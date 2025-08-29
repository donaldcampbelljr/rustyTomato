
# rsPomodoro Timer 🍅

## Run in browser with WASM Bindings

#### Start http server
python3 -m http.server 8000

Then open http://localhost:8000/rspomo.html in your browser.


## Run in terminal with CLI

`cargo build --release`

Set time for 20 minutes
`./target/release/rsPomodoro -n 20 -u m`. 

```
Usage: rsPomodoro [OPTIONS]

Options:
  -t <timer>            [default: timer]
  -n <time_number>      [default: 5]
  -u <time_units>       [default: s]
  -h, --help            Print help
  -V, --version         Print version
```
