use serde::Deserialize;
use sqlx::PgPool;
use std::fs;
use crate::repository::{register_food};

#[derive(Deserialize)]
struct FoodData {
    #[serde(rename = "FoundationFoods")]
    foods: Vec<Option<FoodItem>>,
}

#[derive(Deserialize)]
pub struct FoodItem {
    description: String,
    #[serde(rename = "foodNutrients")]
    nutrients: Vec<FoodNutrient>,
}

#[derive(Deserialize)]
struct FoodNutrient {
    nutrient: NutrientInfo,
    amount: Option<f64>,
}

#[derive(Deserialize)]
struct NutrientInfo {
    number: String,
}

pub async fn conexion_alimentos(pool: &PgPool) -> Result<Vec<FoodItem>, Box<dyn std::error::Error>> {
    let json =
        fs::read_to_string("src/alimentos/FoodData_Central_foundation_food_json_2026-04-30.json").unwrap();
    let data: FoodData = serde_json::from_str(&json)?;
    let foods: Vec<FoodItem> = data.foods.into_iter().flatten().collect();

    for food in &foods {
        let kcal = obtener_calorias(food);
        if let Some(calorias) = kcal {
            register_food(pool, food.description.clone(), calorias.clone()).await?;
        }
    }

    Ok(foods)
}

fn obtener_calorias(food: &FoodItem) -> Option<f64> {
    for nutrient in &food.nutrients {
        if nutrient.nutrient.number == "208" {
            return nutrient.amount;
        }
    }

    None
}