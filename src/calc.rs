use serde::Deserialize;
use std::cell::RefCell;
use std::collections::HashMap;

// Synthetically limit the number of iterations to avoid infinite cycle
const MAX_ITER: usize = 100000;

type CountriesMap = HashMap<String, CountryDimensions>;

#[derive(Deserialize)]
pub struct Input {
    pub id: u64,
    pub countries: CountriesMap,
}

#[derive(Deserialize)]
pub struct CountryDimensions {
    xl: usize,
    yl: usize,
    xh: usize,
    yh: usize,
}

pub struct CountryOutput {
    pub country_name: String,
    pub iter: usize,
}

struct MatrixDimensions {
    rows: usize,
    cols: usize,
}

impl MatrixDimensions {
    /// Get matrix dimensions by finding the bigest x and y for upper right country point
    fn from_map(countries: &HashMap<String, CountryDimensions>) -> MatrixDimensions {
        let rows = countries.values().map(|dim| dim.yh).max().unwrap();
        let cols = countries.values().map(|dim| dim.xh).max().unwrap();

        Self { rows, cols }
    }
}

#[derive(Clone, Default)]
struct City {
    country: String,
    coins: HashMap<String, i64>,
}

/// RefCell is required to enable interior mutability
#[derive(Clone)]
struct Matrix(Vec<Vec<RefCell<City>>>);

impl Matrix {
    fn new(dim: MatrixDimensions, countries: &CountriesMap) -> Matrix {
        let mut matrix = vec![vec![RefCell::<City>::default(); dim.cols + 1]; dim.rows + 1];

        // Initialize the matrix based on country coordinates

        for (cur_country_name, country_dim) in countries {
            for row in country_dim.yl..=country_dim.yh {
                for col in country_dim.xl..=country_dim.xh {
                    let mut city = City {
                        country: cur_country_name.clone(),
                        coins: HashMap::new(),
                    };

                    for country_name in countries.keys().cloned() {
                        // Initial number of the coins for the country that owns the city
                        let coins = if country_name == *cur_country_name {
                            1000000
                        } else {
                            0
                        };
                        city.coins.insert(country_name, coins);
                    }

                    matrix[row][col] = city.into();
                }
            }
        }

        Self(matrix)
    }
}

pub fn euro_diffusion(countries: &CountriesMap) -> Vec<CountryOutput> {
    let country_matrix_dim = MatrixDimensions::from_map(countries);
    let country_matrix = Matrix::new(country_matrix_dim, countries);

    imp_euro_diffusion(country_matrix, countries)
}

fn imp_euro_diffusion(country_matrix: Matrix, countries: &CountriesMap) -> Vec<CountryOutput> {
    if countries.len() == 1 {
        return vec![CountryOutput {
            country_name: countries.keys().next().unwrap().clone(),
            iter: 0,
        }];
    }

    let mut country_matrix = country_matrix.0;
    let transaction_matrix = &mut country_matrix.clone();

    let mut iter = 1;
    let mut completed = Vec::<CountryOutput>::new();

    while iter < MAX_ITER && completed.len() < countries.len() {
        for row in 0..country_matrix.len() {
            for col in 0..country_matrix[row].len() {
                let cur_city = &country_matrix[row][col];
                if cur_city.borrow().country.is_empty() {
                    continue;
                }

                let transaction_cur_city = &transaction_matrix[row][col];

                let transaction_surrounding_cities = [
                    transaction_matrix.get(row - 1).and_then(|row| row.get(col)),
                    transaction_matrix.get(row).and_then(|row| row.get(col - 1)),
                    transaction_matrix.get(row).and_then(|row| row.get(col + 1)),
                    transaction_matrix.get(row + 1).and_then(|row| row.get(col)),
                ]
                .into_iter()
                .flatten();

                for transaction_city in transaction_surrounding_cities {
                    if !transaction_city.borrow().country.is_empty() {
                        // Send to surounging cities coins of EACH country depending on city current balance
                        for country_name in countries.keys() {
                            let value = cur_city.borrow().coins[country_name] / 1000;
                            *transaction_city
                                .borrow_mut()
                                .coins
                                .get_mut(country_name)
                                .unwrap() += value;
                            *transaction_cur_city
                                .borrow_mut()
                                .coins
                                .get_mut(country_name)
                                .unwrap() -= value;
                        }
                    }
                }
            }
        }

        country_matrix = transaction_matrix.clone();

        for country_name in countries.keys().cloned() {
            let country_not_complete = country_matrix
                .iter()
                .flatten()
                .filter(|city| city.borrow().country == country_name)
                .any(|city| {
                    countries
                        .keys()
                        .any(|country_name| city.borrow().coins[country_name] == 0)
                });

            if !country_not_complete
                && !completed
                    .iter()
                    .any(|country| country.country_name == country_name)
            {
                completed.push(CountryOutput { country_name, iter });
            }
        }

        iter += 1;
    }

    completed
}
