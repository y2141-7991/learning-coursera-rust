use polars::prelude::*;

pub fn calculate() -> Result<DataFrame, PolarsError> {
    let df = CsvReader::from_path("data/iris.csv")?
        .has_header(true)
        .finish()?
        .lazy()
        .collect()?;
    Ok(df)
}

fn main() {
    let df = calculate();
    let df1 = df.as_ref().unwrap().select([("species")]).expect("");
    println!("{:?}", df);
    println!("{:?}", df1);
}
