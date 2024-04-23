# Configuring `pclientd`

First, install `pclientd` following the [instructions for installing
`pcli`](../pcli/install.md) but downloading `pclientd` rather than `pcli`.

## Generating configs

`pclientd` can run in either view mode, with only a full viewing key, or custody mode, with the ability to sign transactions.

To initialize `pclientd` in view mode, run
```
pclientd init --view FULL_VIEWING_KEY
```
The `FULL_VIEWING_KEY` can be obtained from the `config.toml` generated by `pcli init`.

To initialize `pclientd` in custody mode, run
```
pclientd init --custody -
```
to read a seed phrase from `stdin`, or
```
pclientd init --custody "SEED PHRASE"
```
to specify the seed phrase on the command line.

## Authorization policy

When run in custody mode, `pclientd` supports configurable authorization policy
for transaction signing.  The default set of policies created by `init
--custody` are an example, and need to be edited before use.

For example, `pclientd init --custody` might generate output like
```toml
full_viewing_key = 'penumbrafullviewingkey1f33fr3zrquh869s3h8d0pjx4fpa9fyut2utw7x5y7xdcxz6z7c8sgf5hslrkpf3mh8d26vufsq8y666chx0x0su06ay3rkwu74zuwqq9w8aza'
grpc_url = 'https://grpc.testnet.penumbra.zone/'
bind_addr = '127.0.0.1:8081'

[kms_config]
spend_key = 'penumbraspendkey1e9gf5g8jfraap4jqul7e80vv0zrnwpsm4ke0df38ejrfh430nu4s9gc22d'

[[kms_config.auth_policy]]
type = 'DestinationAllowList'
allowed_destination_addresses = ['penumbrav2t13vh0fkf3qkqjacpm59g23ufea9n5us45e4p5h6hty8vg73r2t8g5l3kynad87u0n9eragf3hhkgkhqe5vhngq2cw493k48c9qg9ms4epllcmndd6ly4v4dw2jcnxaxzjqnlvnw']

[[kms_config.auth_policy]]
type = 'OnlyIbcRelay'

[[kms_config.auth_policy]]
type = 'PreAuthorization'
method = 'Ed25519'
required_signatures = 1
allowed_signers = ['+Osq5OiWKos57KigDjd3XCG/YLUOSUbuBly4LBBpJTg=']
```

The `kms_config` section controls the configuration of the (software) key
management system.  Each `kms.auth_policy` section is a separate policy that
must be satisfied for transaction authorization to succeed.  To allow any
transaction to be authorized, simply delete all the policies.

### Destination allowlisting
```toml
[[kms_config.auth_policy]]
type = 'DestinationAllowList'
allowed_destination_addresses = ['penumbrav2t13vh0fkf3qkqjacpm59g23ufea9n5us45e4p5h6hty8vg73r2t8g5l3kynad87u0n9eragf3hhkgkhqe5vhngq2cw493k48c9qg9ms4epllcmndd6ly4v4dw2jcnxaxzjqnlvnw']
```
This policy only allows transactions that send funds to the addresses on the
allowlist. Transactions sending funds to any other address will be rejected.

### Relay-only
```toml
[[kms_config.auth_policy]]
type = 'OnlyIbcRelay'
```
This policy only allows transactions with the following actions: `IbcAction`,
`Spend`, `Output`.  The latter two are required to pay fees, so this policy
should be combined with a `DestinationAllowList` to prevent sending funds
outside of the relayer's account.

### Pre-Authorizations
```toml
[[kms_config.auth_policy]]
type = 'PreAuthorization'
method = 'Ed25519'
required_signatures = 1
allowed_signers = ['+Osq5OiWKos57KigDjd3XCG/YLUOSUbuBly4LBBpJTg=']
```
This policy only allows transactions submitted with a pre-authorization Ed25519
signature made with at least `required_signers` signatures from the
`allowed_signers` list.  This allows clients to authenticate authorization
requests to `pclientd` using standard Ed25519 signatures rather than
Penumbra-specific `decaf377-rdsa` signatures.  In the future, more
pre-authorization methods may be added (e.g., WebAuthn).
