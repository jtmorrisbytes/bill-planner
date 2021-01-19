export const INIT = "INIT";
export const GET_TRANSACTIONS = "GET_TRANSACTIONS";
export const GET_TRANSACTIONS_PENDING = "GET_TRANSACTIONS_PENDING";
export const ERROR = "ERROR";
// default to relative urls when not configured
const API_BASE_URL = process.env.API_BASE_URL || "";
export default {
  INIT,
  GET_TRANSACTIONS,
  GET_TRANSACTIONS_PENDING,
  ERROR,
  API_BASE_URL,
};
