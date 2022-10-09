// not compilable yet!

fn main() {
    // inital the TradingBot for coinbase context
let mut coinbase_bot = bot::TradingBot::new(new_config, Box::new(bot::coinbase::Coinbase {}));

// set the interval for every 20s
let trading_cadence = env::var("TRADING_CADENCE").unwrap().parse::<u64>().unwrap();
let mut interval = time::interval(time::Duration::from_secs(trading_cadence));

loop {
  // wait every 20s
  interval.tick().await;

  // trading start time
  let start = Instant::now();
  let now: Date<Local> = Local::now().date();

  // trading kick off
  info!("[TRADE] start at {:?}", now);
  coinbase_bot.start().await.unwrap();

  // trading end time
  let duration = start.elapsed();
  info!("[TRADE] end elapsed : {:?}", duration);
  info!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
}
}
pub struct TradingBot {
    pub trading_config: TradingConfig,
    pub market: Box<dyn Market>,
}

impl TradingBot {
    // main trading logic
    // high sell, low buy
    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let current_price = self.get_market_price().await?;
        info!("[PRICE] current market price: {:?} $", current_price);

        let percentage_diff = (current_price - self.trading_config.last_operation_price)
            / self.trading_config.last_operation_price
            * 100 as f32;

        info!("[PRICE] percentage_diff: {:?} $", percentage_diff);
 
        // based on operation state for the buy and sell action
        match self.trading_config.next_operation {
            State::BUY => {
                self.trading_config.last_operation_price = self.try_to_buy(percentage_diff).await?;
            }
            State::SELL => {
                self.trading_config.last_operation_price =
                    self.try_to_sell(percentage_diff).await?;
            }
        }

        Ok(())
    }
    
    pub fn new(trading_config: TradingConfig, market: Box<dyn Market>) -> Self {
        TradingBot {
            trading_config,
            market,
        }
    }
}
// Trading strategy

// get the buy point
// buy action
async fn try_to_buy(&mut self, diff: f32) -> Result<f32, Box<dyn Error>> {
    if diff >= self.trading_config.upward_trend_threshold
      || diff <= self.trading_config.dip_threshold {
        let current_balance = self.get_balances().await?;
        info!(
          "[BALANCE] current account balance {:?} $ USD",
          current_balance
        );
        self.trading_config.last_operation_price = self.place_buy_order(current_balance).await?;
        // set to the next action to sell
        self.trading_config.next_operation = State::SELL;
        info!(
          "[BUY] Bought 0.002 BTC for {:?} $ USD",
          self.trading_config.last_operation_price
        );
      }
      Ok(self.trading_config.last_operation_price)
  }


pub trait Market {
    async fn get_balances(&self) -> Result<f32, Box<dyn Error>>;
    async fn get_market_price(&self) -> Result<f32, Box<dyn Error>>;
    async fn place_sell_order(&self, amount: f32) -> Result<f32, Box<dyn Error>>;
    async fn place_buy_order(&self, amount: f32) -> Result<f32, Box<dyn Error>>;
}

#[async_trait(?Send)]
impl Market for Coinbase {
    async fn get_balances(&self) -> Result<f32, Box<dyn Error>> {
        // coinbase API call
        Ok(1.0)
    }
    async fn get_market_price(&self) -> Result<f32, Box<dyn Error>> {
        // coinbase API call
        let mut rng = rand::thread_rng();
        Ok(rng.gen_range(0.0, 10.0))
    }

    async fn place_sell_order(&self, amount: f32) -> Result<f32, Box<dyn Error>> {
        // coinbase API call
        Ok(amount)
    }

    async fn place_buy_order(&self, amount: f32) -> Result<f32, Box<dyn Error>> {
        // coinbase API call
        Ok(amount)
    }
}
