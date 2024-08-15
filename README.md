# Get paid for your API.

## TLDR
**`papi` adds a payment gateway to an API.  Choose which cryptocurrencies you would like to be paid in, and how much each API call should cost, and the `papi` integration will handle the rest.**

## Details

### Choice of cryptocurrency
The following cryptocurrencies are supported:

| Name | Token | Comment |
| ---- | ----- | ------- |
| Bitcoin | ckBTC | High speed, low transaction costs are enabled via decentralized chain keys. |
| Bitcoin | ckBTC | High speed, low transaction costs are enabled via decentralized chain keys. |
| Ethereum | ckETH | High speed, low transaction costs are enabled via decentralized chain keys. |
| US Dollar | ckUSDC | High speed, low transaction costs are enabled via decentralized chain keys. |
| ICP Cycles | XDR | The native utility token of the Internet Computer, tied in price to the IMF XDR basket of currencies |
| ICP | ICP | The governance token of the Internet Computer |

### Chain keys: ckBTC, ckEth, ckUSDC, ...
APIs require high speed, low latency and low transaction fees if they are to deliver a good user experience.  Chain Key provides a standard, cryptocurrency-agnostic, decentralized way of delivering these necessary properties.  If you are excited by technical details, you will be glad to know that it enables making L2s on the ICP with threshold keys.  ICP provides the high performance and the threshold keys provide the foundation for making the L2 fully decentralized.
