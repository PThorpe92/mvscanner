use std::fmt::{Display, Formatter};

use crate::database::db::{query, Pool, Query, QueryResult};
use crate::models::locations::Location;
use actix_web::http::{header, StatusCode};
use actix_web::ResponseError;
use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Id {
    pub location_id: usize,
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Location Id: {}", self.location_id)
    }
}

#[derive(Debug, Deserialize)]
pub struct LocationsError(pub String);
impl ResponseError for LocationsError {}

impl std::fmt::Display for LocationsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A validation error occured on the input: {}", self.0)
    }
}

// index all locations
#[rustfmt::skip]
#[get("/api/locations")]
pub async fn index(db: web::Data<Pool>) -> Result<HttpResponse, LocationsError> {
    log::info!("GET: locations controller");
    if let Ok(res) = query(&db, Query::IndexLocations).await {
        match res {
        QueryResult::Locations(locations) => Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(locations)),
        _ => Err(LocationsError("Unable to retrieve locations".to_string())),
        }
    } else {
        Err(LocationsError("Unable to retrieve locations".to_string()))
    }
}
// add a new location
#[rustfmt::skip]
#[post("/api/locations")]
pub async fn store(db: web::Data<Pool>, loc: web::Json<Location>) -> Result<HttpResponse, LocationsError> {
    log::info!("POST: locations controller");
    if let Ok(QueryResult::Success) = query(&db, Query::StoreLocation(&loc.into_inner())).await {
        Ok(HttpResponse::Ok().status(StatusCode::CREATED).insert_header(header::ContentType::json()).json("Location added successfully"))
    } else {
        Err(LocationsError("Unable to add location".to_string()))
    }
}

// Get location name from ID
#[get("/api/locations/{location_id}")]
pub async fn show(db: web::Data<Pool>, id: web::Path<Id>) -> Result<HttpResponse, LocationsError> {
    log::info!("GET: locations controller with id: {}", id.location_id);
    if let Ok(QueryResult::Location(loc)) = query(&db, Query::ShowLocation(id.location_id)).await {
        Ok(HttpResponse::Ok()
            .insert_header(header::ContentType::json())
            .json(loc))
    } else {
        Err(LocationsError("Unable to retrieve location".to_string()))
    }
}

// include range in url to show timestamps from /start/end
#[rustfmt::skip]
#[get("/api/locations/{location_id}/timestamps/{start}/{end}")]
pub async fn show_location_timestamps_range(db: web::Data<Pool>, id: web::Path<(usize, String, String)>) -> Result<HttpResponse, LocationsError> {
    let (id, start, end) = id.into_inner();
    log::info!("GET: Locations controller timestamps with range for ID");
    if let Ok(QueryResult::TimeStamps(ts)) = query(&db, Query::ShowLocationTimestampsRange(id, &start, &end)).await {
        Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(ts))
    } else {
        Err(LocationsError("Unable to retrieve timestamps".to_string()))
    }
}

// show timestamps from today for a location
#[rustfmt::skip]
#[get("/api/locations/{location_id}/timestamps")]
pub async fn show_location_timestamps(db: web::Data<Pool>, id: web::Path<Id>) -> Result<HttpResponse, LocationsError> {
    let id = id.into_inner().location_id;
    log::info!("GET: Locations controller timestamps for ID");
    if let Ok(QueryResult::TimeStamps(ts)) = query(&db, Query::ShowLocationTimestamps(id)).await {
        Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(ts))
    } else {
        Err(LocationsError("Unable to retrieve timestamps".to_string()))
    }
}

// show all residents for a given location
#[rustfmt::skip]
#[get("/api/locations/{location_id}/residents")]
pub async fn show_location_residents(db: web::Data<Pool>, id: web::Path<Id>) -> Result<HttpResponse, LocationsError> {
    let id = id.into_inner().location_id;
    log::info!("GET: Locations controller residents for ID");
    if let Ok(QueryResult::Residents(res)) = query(&db, Query::ShowLocationResidents(id)).await {
        Ok(HttpResponse::Ok()
            .insert_header(header::ContentType::json())
            .json(res))
    } else {
        Err(LocationsError("Unable to retrieve residents".to_string()))
    }
}
