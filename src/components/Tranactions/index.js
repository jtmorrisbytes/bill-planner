import React, { useEffect, useState } from "react";
import decimal from "../../shared/decimal/index.web.js";
import api from "../../api/transactions";
let initialTransactions = [];

class Transaction {
  constructor(amount) {
    if (typeof amount == "string" || typeof amount == "number")
      this.amount = amount;
    else throw new TypeError("Tranaction.amount must be a string or a number");
  }
}

function generateTransactions(countStr, amount) {
  let count = Number(countStr);
  let a = [];
  for (let i = 0; i < count; i++) {
    a.push(new Transaction(amount));
  }
  console.log("generateTransactions result", a);
  return a;
}
function TransactionGenerator(props) {
  const [generateCount, setGenerateCount] = useState("1");
  if (props.progress === true) {
    return <div>generating... please wait!</div>;
  } else
    return (
      <div>
        <input
          type="number"
          onChange={(e) => {
            setGenerateCount(e.target.value);
          }}
          value={generateCount}
        />
        <button
          onClick={async () => {
            props.updateProgress(true);

            await new Promise(async function (resolve, reject) {
              try {
                await props.update(
                  generateTransactions(generateCount, props.amount)
                );
                resolve();
              } catch (e) {
                reject(e);
              }
            }).then(() => {
              props.updateProgress(false);
            });
          }}
        >
          generate
        </button>
      </div>
    );
}

function Transactions(props) {
  // this library returns strings, but can accept numbers
  const [transactions, updateTransactions] = useState(initialTransactions);
  const [total, updateTotal] = useState("0");
  const [amount, updateAmount] = useState("1");
  const [generating, setGenerateProgressIndicator] = useState(false);
  async function updateTransArray(newContent) {
    updateTransactions([...transactions, ...newContent]);
  }

  useEffect(() => {
    api.getTransactions().then((data) => {
      console.log("got transactions from web worker", data);
    });
  });

  useEffect(() => {
    decimal().then((d) => {
      // try {
      console.log(d);
      let result = d.sum_strings_array(transactions.map((t) => t.amount));
      console.log("transaction summ result", result);
      updateTotal(result);
    });
  }, [transactions]);

  return (
    <div>
      <form
        onSubmit={(e) => {
          e.preventDefault();
        }}
      >
        <div style={{ maxHeight: "20vh", overflow: "auto" }}>
          {transactions.map((t) => {
            return <div>{t.amount}</div>;
          })}
        </div>
        <input
          name="amount"
          type="number"
          value={amount}
          onChange={(e) => updateAmount(e.target.value)}
        />
      </form>
      <TransactionGenerator
        updateProgress={setGenerateProgressIndicator}
        progress={generating}
        update={updateTransArray}
        amount={amount}
      ></TransactionGenerator>
      <div>
        <span>total</span>
        <span>{total}</span>
      </div>
    </div>
  );
}

export default Transactions;
