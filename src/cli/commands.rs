pub fn print_commands() {

let commands = r#"{
    "commands":[
      {
          "name":"getinfo",
          "parameters":[],
          "description":"Display general information"
      },
      {
          "name":"getrevocationtxs",
          "parameters":["outpoint"],
          "description":"Retrieve the Revault revocation transactions to sign"
      },  
      {
          "name":"getunvaulttx",
          "parameters":["outpoint"],
          "description":"Retrieve the Revault unvault transaction to sign"
      },
      {
          "name":"getspendtx",
          "parameters":[
              "outpoints",
              "outputs",
              "feerate",
          ],
          "description":"Retrieve the Revault spend transaction to sign"
      },
      {
          "name":"listpresignedtransactions",
          "parameters":[
              "[outpoints]"
          ],
          "description":"List presigned transactions of a confirmed vault"
      },
      {
          "name":"listonchaintransactions",
          "parameters":[
              "[outpoints]"
          ],
          "description":"List broadcast transactions of a vault"
      },
      {
          "name":"listvaults",
          "parameters":[
              "[status]",
              "[outpoints]"
          ],
          "description":"Display a paginated list of vaults"
      },
      {
          "name":"revocationtxs",
          "parameters":[],
          "description":"Give back the revocation transactions signed"
      },
      {
          "name":"unvaulttx",
          "parameters":[],
          "description":"Give back the unvault transaction signed"
      },
      {
          "name":"updatespendtx",
          "parameters":[],
          "description":"Store or update the stored Spend transaction"
      },
      {
          "name":"delspendtx",
          "parameters":[],
          "description":"Delete a stored Spend transaction"
      },
      {
          "name":"setspendtx",
          "parameters":[],
          "description":"Announce and broadcast this Spend transaction"
      },
      {
          "name":"listspendtxs",
          "parameters":[],
          "description":"List all stored Spend transactions"
      },
      {
          "name":"gethistory",
          "parameters":[],
          "description":"Retrieve history of funds"
      },
      {
          "name":"emergency",
          "parameters":[],
          "description":"Broadcast all Emergency signed transactions"
      }
    ]
  }"#;

    println!("{}",commands);
}
  
  
  

  