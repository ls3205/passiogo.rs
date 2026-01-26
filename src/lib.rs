use serde_json::Value;

use crate::helpers::{to_bool, to_f64, to_i64, to_string_opt};
use crate::types::{RouteData, StopData, SystemAlertData, TransportationSystemData, VehicleData};

mod helpers;
mod types;

#[derive(Clone)]
pub struct PassioGoClient {
    base_url: String,
    client: reqwest::Client,
}

impl PassioGoClient {
    pub fn new() -> Self {
        Self {
            base_url: "https://passiogo.com".to_string(),
            client: reqwest::Client::new(),
        }
    }

    async fn send_api_request(
        &self,
        url: &str,
        body: Option<Value>,
    ) -> Result<Value, reqwest::Error> {
        let resp = if let Some(json) = body {
            self.client.post(url).json(&json).send().await?
        } else {
            self.client.get(url).send().await?
        };
        resp.json::<Value>().await
    }

    pub async fn get_systems(&self) -> Result<Vec<TransportationSystemData>, reqwest::Error> {
        let url = format!(
            "{}/mapGetData.php?getSystems=2&sortMode=1&credentials=1",
            self.base_url
        );
        let data = self.send_api_request(&url, None).await?;
        let mut systems = Vec::new();
        let list = data
            .get("all")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        for sys in list {
            let id = sys
                .get("id")
                .and_then(|v| v.as_str())
                .and_then(|v| v.parse::<i64>().ok())
                .or_else(|| sys.get("id").and_then(|v| v.as_i64()))
                .unwrap_or(0);

            systems.push(TransportationSystemData {
                id,
                name: to_string_opt(sys.get("fullname")),
                username: to_string_opt(sys.get("username")),
                go_agency_name: to_string_opt(sys.get("goAgencyName")),
                email: to_string_opt(sys.get("email")),
                go_test_mode: to_bool(sys.get("goTestMode")),
                name2: to_bool(sys.get("name2")),
                homepage: to_string_opt(sys.get("homepage")),
                logo: to_bool(sys.get("logo")),
                go_route_planner_enabled: to_bool(sys.get("goRoutePlannerEnabled")),
                go_color: to_string_opt(sys.get("goColor")),
                go_support_email: to_string_opt(sys.get("goSupportEmail")),
                go_shared_code: to_i64(sys.get("goSharedCode")),
                go_authentication_type: to_bool(sys.get("goAuthenticationType")),
            });
        }
        Ok(systems)
    }

    pub async fn get_alerts(&self, system_id: i64) -> Result<Vec<SystemAlertData>, reqwest::Error> {
        let url = format!("{}/goServices.php?getAlertMessages=1", self.base_url);
        let body = serde_json::json!({
            "systemSelected0": system_id.to_string(),
            "amount": 1
        });

        let data = self.send_api_request(&url, Some(body)).await?;

        let list = if data.is_array() {
            data.as_array().cloned().unwrap_or_default()
        } else {
            data.get("msgs")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default()
        };

        let mut msgs = Vec::new();

        for m in list {
            let id = m
                .get("id")
                .map(|v| v.to_string())
                .unwrap_or_else(|| "".to_string())
                .trim_matches('"')
                .to_string();

            msgs.push(SystemAlertData {
                id,
                system_id: to_i64(m.get("userId")),
                route_id: to_string_opt(m.get("routeId")),
                name: to_string_opt(m.get("name")),
                html: to_string_opt(m.get("html")),
                archive: to_bool(m.get("archive")),
                important: to_bool(m.get("important")),
                date_time_created: to_string_opt(m.get("created")),
                date_time_from: to_string_opt(m.get("from")),
                date_time_to: to_string_opt(m.get("to")),
                as_push: to_bool(m.get("asPush")),
                gtfs: to_bool(m.get("gtfs")),
                gtfs_alert_cause_id: to_i64(m.get("gtfsAlertCauseId")),
                gtfs_alert_effect_id: to_i64(m.get("gtfsAlertEffectId")),
                gtfs_alert_url: to_string_opt(m.get("gtfsAlertUrl")),
                gtfs_alert_header_text: to_string_opt(m.get("gtfsAlertHeaderText")),
                gtfs_alert_description_text: to_string_opt(m.get("gtfsAlertDescriptionText")),
                route_group_id: to_i64(m.get("routeGroupId")),
                created_utc: to_string_opt(m.get("createdUtc")),
                author_id: to_i64(m.get("authorId")),
                author: to_string_opt(m.get("author")),
                updated: to_string_opt(m.get("updated")),
                update_author_id: to_i64(m.get("updateAuthorId")),
                update_author: to_string_opt(m.get("updateAuthor")),
                created_f: to_string_opt(m.get("createdF")),
                from_f: to_string_opt(m.get("fromF")),
                from_ok: to_bool(m.get("fromOk")),
                to_ok: to_bool(m.get("toOk")),
            });
        }

        Ok(msgs)
    }

    pub async fn get_routes(&self, system_id: i64) -> Result<Vec<RouteData>, reqwest::Error> {
        let url = format!("{}/mapGetData.php?getRoutes=1", self.base_url);
        let body = serde_json::json!({
            "systemSelected0": system_id.to_string(),
            "amount": 1
        });
        let data = self.send_api_request(&url, Some(body)).await?;

        let list = if data.is_array() {
            data.as_array().cloned().unwrap_or_default()
        } else {
            data.get("all")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default()
        };

        let mut routes = Vec::new();

        for r in list {
            let id = r
                .get("id")
                .map(|v| v.to_string())
                .unwrap_or_else(|| "".to_string())
                .trim_matches('"')
                .to_string();

            routes.push(RouteData {
                id,
                group_id: to_string_opt(r.get("groupId")),
                group_color: to_string_opt(r.get("groupColor")),
                name: to_string_opt(r.get("name")),
                short_name: to_string_opt(r.get("shortName")),
                name_orig: to_string_opt(r.get("nameOrig")),
                fullname: to_string_opt(r.get("fullname")),
                myid: to_string_opt(r.get("myid")),
                map_app: to_bool(r.get("mapApp")),
                archive: to_bool(r.get("archive")),
                go_prefix_route_name: to_bool(r.get("goPrefixRouteName")),
                go_show_schedule: to_bool(r.get("goShowSchedule")),
                outdated: to_bool(r.get("outdated")),
                distance: to_f64(r.get("distance")),
                latitude: to_f64(r.get("latitude")),
                longitude: to_f64(r.get("longitude")),
                timezone: to_string_opt(r.get("timezone")),
                service_time: to_string_opt(r.get("serviceTime")),
                service_time_short: to_string_opt(r.get("serviceTimeShort")),
                system_id: to_i64(r.get("systemId")),
            });
        }

        Ok(routes)
    }

    pub async fn get_buses(&self, system_id: i64) -> Result<Vec<VehicleData>, reqwest::Error> {
        let url = format!("{}/mapGetData.php?getBuses=2", self.base_url);
        let body = serde_json::json!({
            "s0": system_id.to_string(),
            "sA": 1
        });
        let data = self.send_api_request(&url, Some(body)).await?;

        let buses = data
            .get("buses")
            .and_then(|v| v.as_object())
            .cloned()
            .unwrap_or_default();

        let mut vehicles = Vec::new();

        for (bus_id, record) in buses {
            if bus_id == "-1" {
                continue;
            }

            let list = record.as_array().cloned().unwrap_or_default();

            let v = match list.first() {
                Some(v) => v,
                None => continue,
            };

            let id = v
                .get("busId")
                .map(|v| v.to_string())
                .unwrap_or_else(|| bus_id.clone())
                .trim_matches('"')
                .to_string();

            vehicles.push(VehicleData {
                id,
                name: to_string_opt(v.get("busName")),
                r#type: to_string_opt(v.get("busType")),
                calculated_course: to_f64(v.get("calculatedCourse")),
                route_id: to_string_opt(v.get("routeId")),
                route_name: to_string_opt(v.get("route")),
                color: to_string_opt(v.get("color")),
                created: to_string_opt(v.get("created")),
                latitude: to_f64(v.get("latitude")),
                longitude: to_f64(v.get("longitude")),
                speed: to_f64(v.get("speed")),
                pax_load: to_f64(v.get("paxLoad100")),
                out_of_service: to_bool(v.get("outOfService")),
                more: to_string_opt(v.get("more")),
                trip_id: to_string_opt(v.get("tripId")),
            });
        }

        Ok(vehicles)
    }

    pub async fn get_stops(&self, system_id: i64) -> Result<Vec<StopData>, reqwest::Error> {
        let url = format!("{}/mapGetData.php?getStops=2", self.base_url);
        let body = serde_json::json!({
            "s0": system_id.to_string(),
            "sA": 1
        });
        let data = self.send_api_request(&url, Some(body)).await?;

        let routes = data
            .get("routes")
            .and_then(|v| v.as_object())
            .cloned()
            .unwrap_or_default();

        let stops = data
            .get("stops")
            .and_then(|v| v.as_object())
            .cloned()
            .unwrap_or_default();

        let mut routes_position_map: std::collections::HashMap<String, Vec<(f64, String)>> =
            std::collections::HashMap::new();

        for (route_id, route_val) in routes {
            let list = route_val.as_array().cloned().unwrap_or_default();
            let mut entries = Vec::new();
            for item in list.iter().skip(2) {
                let item_list = match item.as_array() {
                    Some(a) => a,
                    None => continue,
                };
                if item_list.len() < 2 {
                    continue;
                }
                let pos_val = &item_list[0];
                let sid_val = &item_list[1];
                let sid = to_string_opt(Some(sid_val)).unwrap_or_default();
                if sid.is_empty() || sid == "0" {
                    continue;
                }
                let pos = to_f64(Some(pos_val)).unwrap_or(entries.len() as f64);
                entries.push((pos, sid));
            }
            routes_position_map.insert(route_id, entries);
        }

        let mut stop_data = Vec::new();

        for (id, stop) in stops {
            let stop_id = to_string_opt(stop.get("id")).unwrap_or_else(|| id.clone());
            let mut routes_and_positions = std::collections::HashMap::new();
            for (route_id, entries) in routes_position_map.iter() {
                let mut positions = Vec::new();
                for (pos, sid) in entries {
                    if *sid == stop_id {
                        positions.push(*pos);
                    }
                }
                if !positions.is_empty() {
                    routes_and_positions.insert(route_id.clone(), positions);
                }
            }
            stop_data.push(StopData {
                id: stop_id,
                routes_and_positions,
                system_id: to_i64(stop.get("userId")),
                name: to_string_opt(stop.get("name")),
                latitude: to_f64(stop.get("latitude")),
                longitude: to_f64(stop.get("longitude")),
                radius: to_f64(stop.get("radius")),
            });
        }

        Ok(stop_data)
    }
}
