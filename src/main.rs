use calamine::{open_workbook, Xlsx, Reader, RangeDeserializerBuilder, Ods};
use chrono::prelude::*;
use graphql_client::{GraphQLQuery, Response};
use simple_excel_writer::*;
use csv::Writer;
use serde::{Serialize};

use std::error::Error;
use std::env;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "group_schema.graphql",
    query_path = "group_query.graphql",
    response_derives = "Debug"
)]
pub struct Query;

#[derive(Serialize)]
pub struct WBColumn {
    last_name: String,
    first_name:  String,
    group: String,
    level: i64,
    step: i64,
    start_date:  String,
    end_date: String,
    work_hours: f64,
    work_days: f64,
    hourly_rate: f64,
    annual_rate: f64,
    salary: f64,
}

pub fn pay_query(
    identifier1: query::GroupID, 
    level: i64, 
    step: i64, 
    start_date: NaiveDate, 
    end_date: NaiveDate) -> Result<Vec<(String, String, f64, f64, f64, f64, f64)>, Box<dyn Error>> {

    let request_body = Query::build_query(query::Variables {
        identifier1, 
        level, 
        step, 
        start_date, 
        end_date
    });

    let client = reqwest::Client::new();
    let mut res = client.post("https://gc-payscales.herokuapp.com/graphql").json(&request_body).send()?;
    let response_body: Response<query::ResponseData> = res.json()?;

    if let Some(errors) = response_body.errors {
        println!("there are errors:");

        for error in &errors {
            println!("{:?}", error);
        }
    }

    let response_data: query::ResponseData = response_body.data.expect("missing response data");

    let pay_period = response_data.group.pay_at_level_and_step_between_dates.expect("Missing Pay Period");

    let mut response_vec = Vec::new();

    for period in pay_period {
        
        println!("Work Days: {:#?}", period.work_days);
        println!("Work Hours: {:#?}", period.work_hours);
        println!("Hourly Rate: {:#?}", period.hourly_rate);
        println!("Annual Rate: {:#?}", period.annual_rate);
    
        let salary_option = period.salary;
        let salary: f64;
    
        if let Some(s) = salary_option {
            salary = s;
        } else {
            salary = 0.0;
        }
    
        println!("Salary: {:#?}", salary);

        let start = period.start_date.format("%Y-%m-%d").to_string();
        let end = period.end_date.format("%Y-%m-%d").to_string();
    
        response_vec.push((start, end, period.work_days, period.work_hours, period.hourly_rate,
            period.annual_rate, salary));
    
        }
        
        Ok(response_vec)
}

fn main() -> Result<(), calamine::Error> {

    let args: Vec<String> = env::args().collect();

    //let path = "./workbooks/test.ods";
    let open_path = &args[1];
    let save_path = &args[2];

    println!("{}", open_path);
    
    let open_extension: Vec<&str> = open_path.split(".").collect();
    let open_extension: &str = open_extension[2];
    println!("path: {}", open_path);

    println!("extension: {}", open_extension);

    let mut range: calamine::Range<calamine::DataType> = calamine::Range::default();

    match open_extension {
        "ods" => {
            let mut workbook: Ods<_> = open_workbook(open_path)?;
            range = workbook.worksheet_range("Sheet1")
                .ok_or(calamine::Error::Msg("Cannot find sheet"))??;
        }
        "xlsx" => {
            let mut workbook: Xlsx<_> = open_workbook(open_path)?;
            range = workbook.worksheet_range("Sheet1")
                .ok_or(calamine::Error::Msg("Cannot find sheet"))??;
        }
        _ => println!("Not a valid workbook or format.")
    }

    let save_extension: Vec<&str> = save_path.split(".").collect();
    let save_extension: &str = save_extension[1];

    // create iterator over input workbook rows
    let iter = RangeDeserializerBuilder::new().from_range(&range)?;

    let mut data_vec = Vec::new();

    // iterate over input workbook rows
    for result in iter {
        let (
            last_name,
            first_name, 
            group, 
            level, 
            step, 
            start_date, 
            end_date): (String, String, query::GroupID, i64, i64, NaiveDate, NaiveDate) = result?;
            //println!("{} {} {:?} {} {} {} {}", last_name, first_name, group, level, step, start_date, end_date);

            let group_str: String;
            
            {
                group_str = format!("{:?}", &group);
            }
            
            let response_data = pay_query(group, level, step, start_date, end_date).expect("Error on Graphql query");

            for period in response_data {
                
                let (start, end, work_hours, work_days, hourly_rate, annual_rate, salary) = period;
    
                data_vec.push(WBColumn {
                    last_name: last_name.clone(),
                    first_name:  first_name.clone(),
                    group: group_str.clone(),
                    level: level,
                    step: step,
                    start_date:  start,
                    end_date: end,
                    work_hours: work_hours,
                    work_days: work_days,
                    hourly_rate: hourly_rate,
                    annual_rate: annual_rate,
                    salary: salary,
                })
            }

    }

    match save_extension {
        "xlsx" => {
            // create new workbook
            let mut wb = Workbook::create(format!("./workbooks/{}", save_path).as_str());
            let mut sheet = wb.create_sheet("PayUpdate");

            // set column width
            for _ in 0..12 {
                sheet.add_column(Column { width: 10.0 });
            }

            wb.write_sheet(&mut sheet, |sheet_writer| {
                let sw = sheet_writer;
                sw.append_row(row!["last_name", "first_name", "group", "level", "step",
                    "start_date", "end_date", "work_hours", "work_days", "hourly_rate",
                    "annual_rate", "salary"])?;
        
                for e in data_vec {
                    sw.append_row(row![e.last_name.as_str(), e.first_name.as_str(), e.group.as_str(), e.level as f64, e.step as f64,
                        e.start_date, e.end_date, e.work_hours, e.work_days, e.hourly_rate,
                        e.annual_rate, e.salary]).unwrap();
                }
                Ok(())

            }).expect("write excel error!");
        
            wb.close().expect("close excel error!");
        }
        "csv" => {
            let mut wtr = Writer::from_path(format!("./workbooks/{}", save_path)).unwrap();

            for e in data_vec {
                wtr.serialize(e).unwrap();
            }
            wtr.flush()?;
        }
        _ => println!("Not a valid file extension.")
    }

    Ok(())
}
