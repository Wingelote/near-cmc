# Assignment

Make a contract, and write into it every hour the value of bitcoin exchange rate with CMC (https://coinmarketcap.com/).  
The contract stores the last 5 values and outputs the arithmetic average of the price when it is called.


# Setup and build
Please follow the instructions on ![Near documentation](https://docs.near.org/docs/develop/contracts/rust/intro) pages.


# Usage

* Store a rate value

```
near call <YOUR_ACCOUNT_HERE> update_rate --accountId <YOUR_ACCOUNT_HERE> --args '{"rate": 1.0}'
```

* View stored values

```
near call <YOUR_ACCOUNT_HERE> get_collection --accountId <YOUR_ACCOUNT_HERE>
```

* View average rate

```
near call <YOUR_ACCOUNT_HERE> get_average_rate --accountId <YOUR_ACCOUNT_HERE>
```
