use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Reindeer {
    pub name: String,
    pub strength: usize,
    pub speed: Option<f64>,
    pub height: Option<usize>,
    pub antler_width: Option<usize>,
    pub snow_magic_power: Option<usize>,
    pub favorite_food: Option<String>,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    pub candies_eaten_yesterday: Option<usize>,
}

#[post("/4/strength")]
pub async fn four(info: web::Json<Vec<Reindeer>>) -> HttpResponse {
    let mut sum = 0;
    info.iter().for_each(|reindeer| {
        sum += reindeer.strength;
    });

    return HttpResponse::Ok().body(sum.to_string());
}

#[post("/4/contest")]
pub async fn four2(info: web::Json<Vec<Reindeer>>) -> HttpResponse {
    let mut fastest: &Reindeer = &Reindeer::default();
    let mut max_speed = 0f64;
    let mut tallest: &Reindeer = &Reindeer::default();
    let mut max_height = 0;
    let mut widest: &Reindeer = &Reindeer::default();
    let mut max_width = 0;
    let mut most_magic: &Reindeer = &Reindeer::default();
    let mut max_magic = 0;
    let mut hongriest: &Reindeer = &Reindeer::default();
    let mut max_candies = 0;

    info.iter().for_each(|reindeer| {
        let reindeer_speed = reindeer.speed.expect("Should have speed");
        let reindeer_height = reindeer.height.expect("Should have height");
        let reindeer_width = reindeer.antler_width.expect("Should have antler width");
        let reindeer_magic = reindeer.snow_magic_power.expect("Should have magic power");
        let reindeer_candies = reindeer
            .candies_eaten_yesterday
            .expect("Should have candies");

        if reindeer_speed > max_speed {
            fastest = reindeer;
            max_speed = reindeer_speed;
        }

        if reindeer_height > max_height {
            tallest = reindeer;
            max_height = reindeer_height;
        }

        if reindeer_width > max_width {
            widest = reindeer;
            max_width = reindeer_width;
        }

        if reindeer_magic > max_magic {
            most_magic = reindeer;
            max_magic = reindeer_magic;
        }

        if reindeer_candies > max_candies {
            hongriest = reindeer;
            max_candies = reindeer_candies;
        }
    });

    HttpResponse::Ok().body(format! {
        "{{
            \"fastest\": \"Speeding past the finish line with a strength of {} is {}\",
            \"tallest\": \"{} is standing tall with his {} cm wide antlers\",
            \"magician\": \"{} could blast you away with a snow magic power of {}\"
            \"consumer\": \"{} ate lots of candies, but also some {}\"
        }}",
        fastest.strength, fastest.name,
        tallest.name, tallest.antler_width.unwrap(),
        most_magic.name, most_magic.snow_magic_power.unwrap(),
        hongriest.name, hongriest.favorite_food.as_ref().unwrap()
    })
}

