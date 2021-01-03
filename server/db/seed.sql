DROP TABLE IF EXISTS transactions;

CREATE TABLE transactions (
  transaction_id SERIAL PRIMARY KEY,
  user_id TEXT,
  account_number TEXT,
  utc_milli_date BIGINT CHECK (utc_milli_date >=0 AND utc_milli_date < 8639999999999999),
  amount NUMERIC NOT NULL,
  currency TEXT NOT NULL CHECK(currency in ('USD'))
)