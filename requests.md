# Request Processing Flow

This document describes how Nebula processes incoming requests from CometBFT validators. 

## Overview

Nebula's buckets incoming requests into three categories:

- Sign requests (votes, proposals)
- Ping requests
- Pubkey requests

Ping and pubkey requests are not signed, they are for internal usage by the validator; because of this, a response is sent immediately without checks.

Signing requests need to be checked, as Nebula can receive a request for a block that's already been signed.

This document contains the state transitions for both types of requests (signable & non-signable)

### Signable Requests (Proposals & Votes)

```mermaid
stateDiagram-v2
    SignRequest --> CheckedRequest : request.check()
    
    state CheckedRequest {
        [*] --> ValidRequest
        [*] --> DoubleSignVote
        [*] --> DoubleSignProposal
    }
    
    ValidRequest --> PersistCheck : Has new_state
    DoubleSignVote --> DoubleSignVoteResponse : Response_SignedVote_error
    DoubleSignProposal --> DoubleSignProposalResponse : Response_SignedProposal_error
    
    PersistCheck --> PersistSuccess : persist OK
    PersistCheck --> PersistError : persist FAIL
    
    PersistSuccess --> SignedResponse : signer.sign()
    PersistError --> ErrorResponseByVariant : create_error_response()
    
    DoubleSignVoteResponse --> [*] : Send to validator
    DoubleSignProposalResponse --> [*] : Send to validator
    SignedResponse --> [*] : Send to validator
    ErrorResponseByVariant --> [*] : Send to validator
```

### Non-Signable Requests

```mermaid
stateDiagram-v2
    NonSign_Request --> ShowPublicKey : Request_ShowPublicKey
    NonSign_Request --> Ping : Request_Ping
    
    ShowPublicKey --> PublicKeyResponse : Response_PublicKey
    Ping --> PingResponse : Response_Ping
    
    PublicKeyResponse --> [*] : Send to validator
    PingResponse --> [*] : Send to validator
```
