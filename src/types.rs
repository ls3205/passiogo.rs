use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct TransportationSystemData {
    pub id: i64,
    pub name: Option<String>,
    pub username: Option<String>,
    pub go_agency_name: Option<String>,
    pub email: Option<String>,
    pub go_test_mode: Option<bool>,
    pub name2: Option<bool>,
    pub homepage: Option<String>,
    pub logo: Option<bool>,
    pub go_route_planner_enabled: Option<bool>,
    pub go_color: Option<String>,
    pub go_support_email: Option<String>,
    pub go_shared_code: Option<i64>,
    pub go_authentication_type: Option<bool>,
}

#[derive(Default, Debug, Clone)]
pub struct RouteData {
    pub id: String,
    pub group_id: Option<String>,
    pub group_color: Option<String>,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub name_orig: Option<String>,
    pub fullname: Option<String>,
    pub myid: Option<String>,
    pub map_app: Option<bool>,
    pub archive: Option<bool>,
    pub go_prefix_route_name: Option<bool>,
    pub go_show_schedule: Option<bool>,
    pub outdated: Option<bool>,
    pub distance: Option<f64>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub timezone: Option<String>,
    pub service_time: Option<String>,
    pub service_time_short: Option<String>,
    pub system_id: Option<i64>,
}

#[derive(Default, Debug, Clone)]
pub struct StopData {
    pub id: String,
    pub routes_and_positions: HashMap<String, Vec<f64>>,
    pub system_id: Option<i64>,
    pub name: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub radius: Option<f64>,
}

#[derive(Debug, Default, Clone)]
pub struct SystemAlertData {
    pub id: String,
    pub system_id: Option<i64>,
    pub route_id: Option<String>,
    pub name: Option<String>,
    pub html: Option<String>,
    pub archive: Option<bool>,
    pub important: Option<bool>,
    pub date_time_created: Option<String>,
    pub date_time_from: Option<String>,
    pub date_time_to: Option<String>,
    pub as_push: Option<bool>,
    pub gtfs: Option<bool>,
    pub gtfs_alert_cause_id: Option<i64>,
    pub gtfs_alert_effect_id: Option<i64>,
    pub gtfs_alert_url: Option<String>,
    pub gtfs_alert_header_text: Option<String>,
    pub gtfs_alert_description_text: Option<String>,
    pub route_group_id: Option<i64>,
    pub created_utc: Option<String>,
    pub author_id: Option<i64>,
    pub author: Option<String>,
    pub updated: Option<String>,
    pub update_author_id: Option<i64>,
    pub update_author: Option<String>,
    pub created_f: Option<String>,
    pub from_f: Option<String>,
    pub from_ok: Option<bool>,
    pub to_ok: Option<bool>,
}

#[derive(Debug, Default, Clone)]
pub struct VehicleData {
    pub id: String,
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub calculated_course: Option<f64>,
    pub route_id: Option<String>,
    pub route_name: Option<String>,
    pub color: Option<String>,
    pub created: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub speed: Option<f64>,
    pub pax_load: Option<f64>,
    pub out_of_service: Option<bool>,
    pub more: Option<String>,
    pub trip_id: Option<String>,
}
