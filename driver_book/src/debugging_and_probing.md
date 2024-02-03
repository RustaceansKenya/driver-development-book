# debugging_and_probing

A target board exposes a debugging interface.  
So what is a debugging interface? 


## probe-rs 

#### basic info:
- probe-rs list  // for you to see the respective vids and pids
- probe-rs info  // on the first debugger. But it might fail because the probe assumes SWD is being used
- probe-rs info --probe 303a:1001 --protocol jtag   // --probe vendorID:productID  --protocol [jtag OR SWD] 

#### Taking action


#### Questions
- How can I add custom chips?
- 