use calamine::{open_workbook, Xlsx, Reader, RangeDeserializerBuilder, Ods};
use chrono::prelude::*;
use graphql_client::{GraphQLQuery, Response};
use std::error::Error;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "group_schema.graphql",
    query_path = "group_query.graphql",
    response_derives = "Debug"
)]
pub struct GroupQuery;

pub fn pay_query(identifier1: group_query::GroupID, level: i64, step: i64, start_date: NaiveDate, end_date: NaiveDate) -> Result<(), Box<dyn Error>> {

    let request_body = GroupQuery::build_query(group_query::Variables {
        identifier1, 
        level, 
        step, 
        start_date, 
        end_date
    });

    let client = reqwest::Client::new();
    let mut res = client.post("https://gc-payscales.herokuapp.com/graphql").json(&request_body).send()?;
    let response_body: Response<group_query::ResponseData> = res.json()?;

    println!("{:#?}", response_body);
    Ok(())

}

fn main() -> Result<(), calamine::Error> {
    let path = "./workbooks/test.ods";
    let mut workbook: Ods<_> = open_workbook(path)?;
    let range = workbook.worksheet_range("Sheet1")
        .ok_or(calamine::Error::Msg("Cannot find sheet"))??;

    let iter = RangeDeserializerBuilder::new().from_range(&range)?;

    for result in iter {
        let (
            last_name,
            first_name, 
            group, 
            level, 
            step, 
            start_date, 
            end_date): (String, String, group_query::GroupID, i64, i64, NaiveDate, NaiveDate) = result?;
            println!("{} {} {:?} {} {} {} {}", last_name, first_name, group, level, step, start_date, end_date);

            pay_query(group, level, step, start_date, end_date).expect("Error on Graphql query");
    }

    Ok(())
}
