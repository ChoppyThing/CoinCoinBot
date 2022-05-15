pub mod database {
	use serde::Deserialize;
	use rusqlite::{Connection, Result};

	#[derive(Deserialize, Debug)]
	pub struct Timestamp {
		pub id: i32,
		pub name: String,
		pub direction: String,
		pub value: f64,
		pub datetime: String
	}

	fn open() -> Connection {
		let conn = match Connection::open("transaction.db") {
			Ok(a) => a,
			Err(_) => panic!("Database cannot be opened or created.")
		};

		let mut _result = conn.execute("CREATE TABLE IF NOT EXISTS timestamp (
				id INTEGER PRIMARY KEY AUTOINCREMENT,
	            name TEXT NOT NULL,
				direction TEXT NOT NULL, /* BUY or SELL */
	            value DECIMAL(10, 2) NOT NULL,
	            datetime TEXT NOT NULL
	        )",
	        [],
	    );

	    _result = conn.execute("CREATE TABLE IF NOT EXISTS stock (
	            id SERIAL PRIMARY KEY,
	            name TEXT NOT NULL,
	            type TEXT NOT NULL,
	            value DECIMAL(10,2) NOT NULL,
				bought_at DECIMAL(10,2) NOT NULL,
				sold_at DECIMAL(10,2) NULL,
	            status TEXT NOT NULL,
	            datetime TEXT NOT NULL
	        )",
	        [],
	    );

	    conn
	}

	// pub fn insert() -> Result<()> {
	// 	let conn = open();
	// 	let mut statement = conn.prepare(
	// 		"INSERT INTO timestamp (name, value) VALUES (:name, :value)"
	// 	)?;
	// 	let _test = statement.execute(&[(":name", "Test"), (":value", "50.36")])?;

	// 	Ok(())
	// }

	pub fn add_timestamp(name: String, direction: String, value: String, datetime: String) -> Result<()> {
		let conn = open();
		let mut statement = conn.prepare(
			"INSERT INTO timestamp (name, direction, value, datetime) VALUES (:name, :direction, :value, :datetime)"
		)?;
		let _test = statement.execute(&[
			(":name", &name),
			(":direction", &direction),
			(":value", &value),
			(":datetime", &datetime),
		])?;

		Ok(())
	}

	pub fn last_sell_prices(check_period: &str, name: &str) -> Vec<Timestamp> {
		let conn = open();
		let mut statement = conn.prepare(
			"SELECT * FROM timestamp
			WHERE datetime > :check_period
			AND direction = 'BUY_AT'
			AND name = :name
			ORDER BY id ASC"
		).expect("Statement error");

		let timestamp_list = statement.query_map(&[
			(":check_period", check_period),
			(":name", name),
		], |row| {
			Ok(Timestamp {
				id: row.get(0)?,
				name: row.get(1)?,
				direction: row.get(2)?,
				value: row.get(3)?,
				datetime: row.get(4)?,
			})
		}).expect("Query error");

		let mut timestamps: Vec<Timestamp> = Vec::new();
		for timestamp in timestamp_list {
			let stamp: Timestamp = timestamp.unwrap();
			timestamps.push(stamp);
		}

		timestamps
	}

	pub fn get_last_sell_price(name: &str) -> Timestamp {
		let conn = open();
		let mut statement = conn.prepare(
			"SELECT * FROM timestamp
			WHERE direction = 'BUY_AT'
			AND name = :name
			ORDER BY id DESC
			LIMIT 1"
		).expect("Statement error");

		let timestamp_list = statement.query_map(&[(":name", name)], |row| {
			Ok(Timestamp {
				id: row.get(0)?,
				name: row.get(1)?,
				direction: row.get(2)?,
				value: row.get(3)?,
				datetime: row.get(4)?,
			})
		}).expect("Query error");

		timestamp_list.last().unwrap().expect("Last sell price not found")
	}
}
