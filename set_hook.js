if (process.argv.length < 4)
{
    console.log("Usage: node set_hook <account family seed> hook-name")
    process.exit(1);
}
const RippleAPI = require('ripple-lib').RippleAPI;
const keypairs = require('ripple-keypairs');
const fs = require('fs');
const api = new RippleAPI({server: 'wss://hooks-testnet.xrpl-labs.com'});

const secret  = process.argv[2];
const address = keypairs.deriveAddress(keypairs.deriveKeypair(secret).publicKey)
console.log(address);

const hook_name = process.argv[3]
const paht_to_wasm = hook_name + '.wasm'
console.log(paht_to_wasm);

api.on('error', (errorCode, errorMessage) => {console.log(errorCode + ': ' + errorMessage);});
api.on('connected', () => {console.log('connected');});
api.on('disconnected', (code) => {console.log('disconnected, code:', code);});
api.connect().then(() => {
    binary = fs.readFileSync(paht_to_wasm).toString('hex').toUpperCase();
    j = {
        Account: address,
        TransactionType: "SetHook",
        CreateCode: binary,
        HookOn: '0000000000000000'
    }
    api.prepareTransaction(j).then( (x)=> 
    {
        //console.log(x.txJSON)
        let s = api.sign(x.txJSON, secret)
        //console.log(s)
        api.submit(s.signedTransaction).then( response => {
            console.log(response.resultCode, response.resultMessage);
            process.exit(0);        
        }).catch ( e=> { console.log(e) });
    });
}).then(() => {}).catch(console.error);
