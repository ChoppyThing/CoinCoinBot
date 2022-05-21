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

	#[derive(Deserialize, Debug)]
	pub struct Stock {
		pub id: i32,
		pub name: String,
		pub amount: f64,
		pub bought_at: f64,
		pub sold_at: f64,
		pub status: String,
		pub datetime: String,
		pub fees: f64,
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
	            id INTEGER PRIMARY KEY AUTOINCREMENT,
	            name TEXT NOT NULL,
	            amount DECIMAL(10,2) NOT NULL,
				bought_at DECIMAL(10,2) NOT NULL,
				sold_at DECIMAL(10,2) NULL,
	            status TEXT NOT NULL,
	            datetime TEXT NOT NULL,
				fees DECIMAL(10,2) NOT NULL
	        )",
	        [],
	    );

	    conn
	}

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

	pub fn buy_stock(name: String, amount: String, bought_at: String, fees: String) -> Result<()> {
		let datetime = chrono::offset::Local::now();
        let now = datetime.format("%F %T").to_string();

		let conn = open();
		let mut statement = conn.prepare(
			"INSERT INTO stock (name, amount, bought_at, sold_at, status, datetime, fees)
			VALUES (:name, :amount, :bought_at, 0, :status, :datetime, :fees)"
		)?;
		let _test = statement.execute(&[
			(":name", &name),
			(":amount", &amount),
			(":bought_at", &bought_at),
			(":status", &"BOUGHT".to_string()),
			(":datetime", &now.to_string()),
			(":fees", &fees)
		])?;

		Ok(())
	}

	pub fn set_sold_stock(id: String, total: String) -> Result<()> {
		let conn = open();
		let mut statement = conn.prepare(
			"UPDATE stock
				SET status='SOLD',
				sold_at=:sold_at
			WHERE id=:id"
		)?;
		let _test = statement.execute(&[
			(":id", &id),
			(":sold_at", &total),
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

	pub fn get_last_price(name: &str, direction: &str) -> Timestamp {
		let conn = open();
		let mut statement = conn.prepare(
			"SELECT * FROM timestamp
			WHERE direction = :direction
			AND name = :name
			ORDER BY id DESC
			LIMIT 1"
		).expect("Statement error");

		let timestamp_list = statement.query_map(&[
			(":name", name),
			(":direction", direction)], |row| {
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

	pub fn get_last_unsold_stock(name: &str) -> Option<Stock> {
		let conn = open();
		let mut statement = conn.prepare(
			"SELECT * FROM stock
			WHERE name = :name
			AND status = :status
			ORDER BY id DESC
			LIMIT 1"
		).expect("Statement error");

		let stock = statement.query_map(&[
			(":name", name),
			(":status", &"BOUGHT".to_string())], |row| {
			Ok(Stock {
				id: row.get(0)?,
				name: row.get(1)?,
				amount: row.get(2)?,
				bought_at: row.get(3)?,
				sold_at: row.get(4)?,
				status: row.get(5)?,
				datetime: row.get(6)?,
				fees: row.get(7)?,
			})
		}).expect("test");

		stock.last().map(|result| result.ok()).flatten()
	}

	pub fn get_unsold_stock(name: &str) -> Option<Vec<Stock>> {
		let conn = open();
		let mut statement = conn.prepare(
			"SELECT * FROM stock
			WHERE name = :name
			AND status = :status"
		).expect("Statement error");

		let stock = statement.query_map(&[
			(":name", name),
			(":status", &"BOUGHT".to_string())], |row| {
			Ok(Stock {
				id: row.get(0)?,
				name: row.get(1)?,
				amount: row.get(2)?,
				bought_at: row.get(3)?,
				sold_at: row.get(4)?,
				status: row.get(5)?,
				datetime: row.get(6)?,
				fees: row.get(7)?,
			})
		}).expect("test");

		let mut lines: Vec<Stock> = Vec::new();
		for line in stock {
			lines.push(line.unwrap());
		}

		Some(lines)
	}
}
