import React,{useEffect,useState} from "react"
import decimal from "../../shared/decimal/index.web.js"
let initialTransactions = []

class Transaction {
    constructor(amount){
        if(typeof amount == "string" || typeof amount == "number")
        this.amount = amount;
        else throw new TypeError("Tranaction.amount must be a string or a number")
    }
}


function Transactions(props){
    // this library returns strings, but can accept numbers
    const [transactions,updateTransactions] = useState(initialTransactions)
    const [total,updateTotal] = useState("0")
    useEffect(()=>{

        decimal().then((d)=>{
            // try {
                console.log(d)
                let result =    d.sum_strings_array(transactions.map((t)=>t.amount)) 
                console.log("transaction summ result", result)
            updateTotal(result)
            
            
                
            
        })
    },[transactions])

    return <div>
        <form onSubmit={(e)=>{
            e.preventDefault()
            updateTransactions([...transactions,new Transaction(e.target.elements['amount'].value)])
            }}>
            <div>
                {transactions.map((t)=>{
                    return <div>{t.amount}</div>
                })}
            </div>
            <input name="amount" type='number' />
            <button formAction="submit">add transaction</button>
        </form>
        <div>
            <span>total</span>
            <span>{total}</span>
        </div>
    </div>
}

export default Transactions