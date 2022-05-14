pub mod database {
	use rusqlite::{Connection, Result};

	pub fn open() -> Connection {
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
	            status TEXT NOT NULL,
	            datetime TEXT NOT NULL
	        )",
	        [],
	    );

	    conn
	}

	pub fn insert() -> Result<()> {
		let conn = open();
		let mut statement = conn.prepare(
			"INSERT INTO timestamp (name, value) VALUES (:name, :value)"
		)?;
		let _test = statement.execute(&[(":name", "Test"), (":value", "50.36")])?;

		Ok(())
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
}
