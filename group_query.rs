pub struct GroupQuery;
pub mod group_query {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &'static str = "GroupQuery";
    pub const QUERY : & 'static str = "query GroupQuery(\n  $identifier1:GroupID!,\n  $level: Int!, \n  $step:Int!,\n\t$startDate: NaiveDate!,\n\t$endDate:NaiveDate!) {\n  group(identifier: $identifier1) {\n    payscaleForLevel(level:$level) {\n      steps\n    }\n    identifier\n    payAtLevelAndStepBetweenDates(\n    \tlevel: $level\n      step: $step\n      startDate:$startDate\n      endDate:$endDate\n    ){\n      startDate\n      endDate\n      workDays\n      workHours\n      hourlyRate\n      annualRate\n      salary\n    }\n  }\n}" ;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[doc = "NaiveDate"]
    type NaiveDate = super::NaiveDate;
    #[derive(Eq, PartialEq)]
    pub enum GroupID {
        #[doc = "GroupID represents a two-letter identifier for a pay group as an enum"]
        CS,
        CX,
        DS,
        LS,
        EC,
        EL,
        FB,
        FI,
        FS,
        AS,
        CM,
        CR,
        IS,
        PM,
        WP,
        HR,
        RO,
        DE,
        OP,
        PH,
        PS,
        VM,
        AC,
        AG,
        BI,
        CH,
        FO,
        PC,
        FR,
        LI,
        PR,
        SC,
        DD,
        EG,
        GT,
        PI,
        PY,
        TI,
        TR,
        UT,
        Other(String),
    }
    impl ::serde::Serialize for GroupID {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                GroupID::CS => "CS",
                GroupID::CX => "CX",
                GroupID::DS => "DS",
                GroupID::LS => "LS",
                GroupID::EC => "EC",
                GroupID::EL => "EL",
                GroupID::FB => "FB",
                GroupID::FI => "FI",
                GroupID::FS => "FS",
                GroupID::AS => "AS",
                GroupID::CM => "CM",
                GroupID::CR => "CR",
                GroupID::IS => "IS",
                GroupID::PM => "PM",
                GroupID::WP => "WP",
                GroupID::HR => "HR",
                GroupID::RO => "RO",
                GroupID::DE => "DE",
                GroupID::OP => "OP",
                GroupID::PH => "PH",
                GroupID::PS => "PS",
                GroupID::VM => "VM",
                GroupID::AC => "AC",
                GroupID::AG => "AG",
                GroupID::BI => "BI",
                GroupID::CH => "CH",
                GroupID::FO => "FO",
                GroupID::PC => "PC",
                GroupID::FR => "FR",
                GroupID::LI => "LI",
                GroupID::PR => "PR",
                GroupID::SC => "SC",
                GroupID::DD => "DD",
                GroupID::EG => "EG",
                GroupID::GT => "GT",
                GroupID::PI => "PI",
                GroupID::PY => "PY",
                GroupID::TI => "TI",
                GroupID::TR => "TR",
                GroupID::UT => "UT",
                GroupID::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GroupID {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "CS" => Ok(GroupID::CS),
                "CX" => Ok(GroupID::CX),
                "DS" => Ok(GroupID::DS),
                "LS" => Ok(GroupID::LS),
                "EC" => Ok(GroupID::EC),
                "EL" => Ok(GroupID::EL),
                "FB" => Ok(GroupID::FB),
                "FI" => Ok(GroupID::FI),
                "FS" => Ok(GroupID::FS),
                "AS" => Ok(GroupID::AS),
                "CM" => Ok(GroupID::CM),
                "CR" => Ok(GroupID::CR),
                "IS" => Ok(GroupID::IS),
                "PM" => Ok(GroupID::PM),
                "WP" => Ok(GroupID::WP),
                "HR" => Ok(GroupID::HR),
                "RO" => Ok(GroupID::RO),
                "DE" => Ok(GroupID::DE),
                "OP" => Ok(GroupID::OP),
                "PH" => Ok(GroupID::PH),
                "PS" => Ok(GroupID::PS),
                "VM" => Ok(GroupID::VM),
                "AC" => Ok(GroupID::AC),
                "AG" => Ok(GroupID::AG),
                "BI" => Ok(GroupID::BI),
                "CH" => Ok(GroupID::CH),
                "FO" => Ok(GroupID::FO),
                "PC" => Ok(GroupID::PC),
                "FR" => Ok(GroupID::FR),
                "LI" => Ok(GroupID::LI),
                "PR" => Ok(GroupID::PR),
                "SC" => Ok(GroupID::SC),
                "DD" => Ok(GroupID::DD),
                "EG" => Ok(GroupID::EG),
                "GT" => Ok(GroupID::GT),
                "PI" => Ok(GroupID::PI),
                "PY" => Ok(GroupID::PY),
                "TI" => Ok(GroupID::TI),
                "TR" => Ok(GroupID::TR),
                "UT" => Ok(GroupID::UT),
                _ => Ok(GroupID::Other(s)),
            }
        }
    }
    #[derive(Deserialize)]
    #[doc = "A payscale containing a specific level and agreed pay rates for a period of time and pay steps.\nThis would contain all data for an EC-04, for example, including the changes to pay according to the collective agreeement\nand the annual pay steps within the specific agreement.\nOf note, many payScales are behind the current date and/or are being negotiated at any point in time."]
    pub struct GroupQueryGroupPayscaleForLevel {
        #[doc = "The number of steps in a payscale - e.g., there are 5 steps in the EC-04 payscale"]
        pub steps: Int,
    }
    #[derive(Deserialize)]
    #[doc = "IMPORTANT: IN TESTING\nA Pay Period represents a series of time periods and the approximate gross pay expect in each period\nIt is based on the pay rate in force for the dates in question at a current level and step.\nIf you want to track different steps, you will need to run multiple instances of PayAtLevelAndStepBetweenDates\nin your query."]
    pub struct GroupQueryGroupPayAtLevelAndStepBetweenDates {
        #[doc = "The start date for a pay period in YYYY-MM-DD."]
        #[serde(rename = "startDate")]
        pub start_date: NaiveDate,
        #[doc = "The end date for a pay period in YYYY-MM-DD."]
        #[serde(rename = "endDate")]
        pub end_date: NaiveDate,
        #[doc = "The duration in days for a pay period."]
        #[serde(rename = "workDays")]
        pub work_days: Float,
        #[doc = "The duration in hours for a pay period."]
        #[serde(rename = "workHours")]
        pub work_hours: Float,
        #[doc = "The hourly pay rate for a pay period."]
        #[serde(rename = "hourlyRate")]
        pub hourly_rate: Float,
        #[doc = "The annual pay rate for a pay period."]
        #[serde(rename = "annualRate")]
        pub annual_rate: Float,
        #[doc = "The gross salary (approximate) for a pay period"]
        pub salary: Option<Float>,
    }
    #[derive(Deserialize)]
    #[doc = "A pay group as defined by a collective agreement"]
    pub struct GroupQueryGroup {
        #[doc = "Returns a payscale for a specific level within the group."]
        #[serde(rename = "payscaleForLevel")]
        pub payscale_for_level: Option<GroupQueryGroupPayscaleForLevel>,
        #[doc = "The two-letter identifier for the group as an enum."]
        pub identifier: GroupID,
        #[doc = "Returns a vector of PayPeriods representing the expected pay for a range of work days inclusive of two YYYY-MM-DD dates.\nFor example, start_date: \"2020-05-01\" and end_date: \"2020-05-05\" would return pay for 1 day of 7.5 hours.\nThe function returns work days and holidays (for which public servants receive pay), but not weekends.\nAlso requires a level and step in integers to compute the requested pay."]
        #[serde(rename = "payAtLevelAndStepBetweenDates")]
        pub pay_at_level_and_step_between_dates:
            Option<Vec<GroupQueryGroupPayAtLevelAndStepBetweenDates>>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub identifier1: GroupID,
        pub level: Int,
        pub step: Int,
        #[serde(rename = "startDate")]
        pub start_date: NaiveDate,
        #[serde(rename = "endDate")]
        pub end_date: NaiveDate,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub group: GroupQueryGroup,
    }
}
impl graphql_client::GraphQLQuery for GroupQuery {
    type Variables = group_query::Variables;
    type ResponseData = group_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: group_query::QUERY,
            operation_name: group_query::OPERATION_NAME,
        }
    }
}
