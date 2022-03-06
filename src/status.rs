#[derive(FromPrimitive, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Status {
    /// The transaction passed the precheck validations.
    Ok = 0,
    ///*
    /// For any error not handled by specific error codes listed below.
    InvalidTransaction = 1,
    ///*
    /// Payer account does not exist.
    PayerAccountNotFound = 2,
    ///*
    /// Node Account provided does not match the node account of the node the transaction was submitted
    /// to.
    InvalidNodeAccount = 3,
    ///*
    /// Pre-Check error when TransactionValidStart + transactionValidDuration is less than current
    /// consensus time.
    TransactionExpired = 4,
    ///*
    /// Transaction start time is greater than current consensus time
    InvalidTransactionStart = 5,
    ///*
    /// The given transactionValidDuration was either non-positive, or greater than the maximum
    /// valid duration of 180 secs.
    ///
    InvalidTransactionDuration = 6,
    ///*
    /// The transaction signature is not valid
    InvalidSignature = 7,
    ///*
    /// Transaction memo size exceeded 100 bytes
    MemoTooLong = 8,
    ///*
    /// The fee provided in the transaction is insufficient for this type of transaction
    InsufficientTxFee = 9,
    ///*
    /// The payer account has insufficient cryptocurrency to pay the transaction fee
    InsufficientPayerBalance = 10,
    ///*
    /// This transaction ID is a duplicate of one that was submitted to this node or reached consensus
    /// in the last 180 seconds (receipt period)
    DuplicateTransaction = 11,
    ///*
    /// If API is throttled out
    Busy = 12,
    ///*
    /// The API is not currently supported
    NotSupported = 13,
    ///*
    /// The file id is invalid or does not exist
    InvalidFileId = 14,
    ///*
    /// The account id is invalid or does not exist
    InvalidAccountId = 15,
    ///*
    /// The contract id is invalid or does not exist
    InvalidContractId = 16,
    ///*
    /// Transaction id is not valid
    InvalidTransactionId = 17,
    ///*
    /// Receipt for given transaction id does not exist
    ReceiptNotFound = 18,
    ///*
    /// Record for given transaction id does not exist
    RecordNotFound = 19,
    ///*
    /// The solidity id is invalid or entity with this solidity id does not exist
    InvalidSolidityId = 20,
    ///*
    /// The responding node has submitted the transaction to the network. Its final status is still
    /// unknown.
    Unknown = 21,
    ///*
    /// The transaction succeeded
    Success = 22,
    ///*
    /// There was a system error and the transaction failed because of invalid request parameters.
    FailInvalid = 23,
    ///*
    /// There was a system error while performing fee calculation, reserved for future.
    FailFee = 24,
    ///*
    /// There was a system error while performing balance checks, reserved for future.
    FailBalance = 25,
    ///*
    /// Key not provided in the transaction body
    KeyRequired = 26,
    ///*
    /// Unsupported algorithm/encoding used for keys in the transaction
    BadEncoding = 27,
    ///*
    /// When the account balance is not sufficient for the transfer
    InsufficientAccountBalance = 28,
    ///*
    /// During an update transaction when the system is not able to find the Users Solidity address
    InvalidSolidityAddress = 29,
    ///*
    /// Not enough gas was supplied to execute transaction
    InsufficientGas = 30,
    ///*
    /// contract byte code size is over the limit
    ContractSizeLimitExceeded = 31,
    ///*
    /// local execution (query) is requested for a function which changes state
    LocalCallModificationException = 32,
    ///*
    /// Contract REVERT OPCODE executed
    ContractRevertExecuted = 33,
    ///*
    /// For any contract execution related error not handled by specific error codes listed above.
    ContractExecutionException = 34,
    ///*
    /// In Query validation, account with +ve(amount) value should be Receiving node account, the
    /// receiver account should be only one account in the list
    InvalidReceivingNodeAccount = 35,
    ///*
    /// Header is missing in Query request
    MissingQueryHeader = 36,
    ///*
    /// The update of the account failed
    AccountUpdateFailed = 37,
    ///*
    /// Provided key encoding was not supported by the system
    InvalidKeyEncoding = 38,
    ///*
    /// null solidity address
    NullSolidityAddress = 39,
    ///*
    /// update of the contract failed
    ContractUpdateFailed = 40,
    ///*
    /// the query header is invalid
    InvalidQueryHeader = 41,
    ///*
    /// Invalid fee submitted
    InvalidFeeSubmitted = 42,
    ///*
    /// Payer signature is invalid
    InvalidPayerSignature = 43,
    ///*
    /// The keys were not provided in the request.
    KeyNotProvided = 44,
    ///*
    /// Expiration time provided in the transaction was invalid.
    InvalidExpirationTime = 45,
    ///*
    /// WriteAccess Control Keys are not provided for the file
    NoWaclKey = 46,
    ///*
    /// The contents of file are provided as empty.
    FileContentEmpty = 47,
    ///*
    /// The crypto transfer credit and debit do not sum equal to 0
    InvalidAccountAmounts = 48,
    ///*
    /// Transaction body provided is empty
    EmptyTransactionBody = 49,
    ///*
    /// Invalid transaction body provided
    InvalidTransactionBody = 50,
    ///*
    /// the type of key (base ed25519 key, KeyList, or ThresholdKey) does not match the type of
    /// signature (base ed25519 signature, SignatureList, or ThresholdKeySignature)
    InvalidSignatureTypeMismatchingKey = 51,
    ///*
    /// the number of key (KeyList, or ThresholdKey) does not match that of signature (SignatureList,
    /// or ThresholdKeySignature). e.g. if a keyList has 3 base keys, then the corresponding
    /// signatureList should also have 3 base signatures.
    InvalidSignatureCountMismatchingKey = 52,
    ///*
    /// the livehash body is empty
    EmptyLiveHashBody = 53,
    ///*
    /// the livehash data is missing
    EmptyLiveHash = 54,
    ///*
    /// the keys for a livehash are missing
    EmptyLiveHashKeys = 55,
    ///*
    /// the livehash data is not the output of a SHA-384 digest
    InvalidLiveHashSize = 56,
    ///*
    /// the query body is empty
    EmptyQueryBody = 57,
    ///*
    /// the crypto livehash query is empty
    EmptyLiveHashQuery = 58,
    ///*
    /// the livehash is not present
    LiveHashNotFound = 59,
    ///*
    /// the account id passed has not yet been created.
    AccountIdDoesNotExist = 60,
    ///*
    /// the livehash already exists for a given account
    LiveHashAlreadyExists = 61,
    ///*
    /// File WACL keys are invalid
    InvalidFileWacl = 62,
    ///*
    /// Serialization failure
    SerializationFailed = 63,
    ///*
    /// The size of the Transaction is greater than transactionMaxBytes
    TransactionOversize = 64,
    ///*
    /// The Transaction has more than 50 levels
    TransactionTooManyLayers = 65,
    ///*
    /// Contract is marked as deleted
    ContractDeleted = 66,
    ///*
    /// the platform node is either disconnected or lagging behind.
    PlatformNotActive = 67,
    ///*
    /// one public key matches more than one prefixes on the signature map
    KeyPrefixMismatch = 68,
    ///*
    /// transaction not created by platform due to large backlog
    PlatformTransactionNotCreated = 69,
    ///*
    /// auto renewal period is not a positive number of seconds
    InvalidRenewalPeriod = 70,
    ///*
    /// the response code when a smart contract id is passed for a crypto API request
    InvalidPayerAccountId = 71,
    ///*
    /// the account has been marked as deleted
    AccountDeleted = 72,
    ///*
    /// the file has been marked as deleted
    FileDeleted = 73,
    ///*
    /// same accounts repeated in the transfer account list
    AccountRepeatedInAccountAmounts = 74,
    ///*
    /// attempting to set negative balance value for crypto account
    SettingNegativeAccountBalance = 75,
    ///*
    /// when deleting smart contract that has crypto balance either transfer account or transfer smart
    /// contract is required
    ObtainerRequired = 76,
    ///*
    /// when deleting smart contract that has crypto balance you can not use the same contract id as
    /// transferContractId as the one being deleted
    ObtainerSameContractId = 77,
    ///*
    /// transferAccountId or transferContractId specified for contract delete does not exist
    ObtainerDoesNotExist = 78,
    ///*
    /// attempting to modify (update or delete a immutable smart contract, i.e. one created without a
    /// admin key)
    ModifyingImmutableContract = 79,
    ///*
    /// Unexpected exception thrown by file system functions
    FileSystemException = 80,
    ///*
    /// the duration is not a subset of [MINIMUM_AUTORENEW_DURATION,MAXIMUM_AUTORENEW_DURATION]
    AutorenewDurationNotInRange = 81,
    ///*
    /// Decoding the smart contract binary to a byte array failed. Check that the input is a valid hex
    /// string.
    ErrorDecodingBytestring = 82,
    ///*
    /// File to create a smart contract was of length zero
    ContractFileEmpty = 83,
    ///*
    /// Bytecode for smart contract is of length zero
    ContractBytecodeEmpty = 84,
    ///*
    /// Attempt to set negative initial balance
    InvalidInitialBalance = 85,
    ///*
    /// [Deprecated]. attempt to set negative receive record threshold
    InvalidReceiveRecordThreshold = 86,
    ///*
    /// [Deprecated]. attempt to set negative send record threshold
    InvalidSendRecordThreshold = 87,
    ///*
    /// Special Account Operations should be performed by only Genesis account, return this code if it
    /// is not Genesis Account
    AccountIsNotGenesisAccount = 88,
    ///*
    /// The fee payer account doesn't have permission to submit such Transaction
    PayerAccountUnauthorized = 89,
    ///*
    /// FreezeTransactionBody is invalid
    InvalidFreezeTransactionBody = 90,
    ///*
    /// FreezeTransactionBody does not exist
    FreezeTransactionBodyNotFound = 91,
    ///*
    /// Exceeded the number of accounts (both from and to) allowed for crypto transfer list
    TransferListSizeLimitExceeded = 92,
    ///*
    /// Smart contract result size greater than specified maxResultSize
    ResultSizeLimitExceeded = 93,
    ///*
    /// The payer account is not a special account(account 0.0.55)
    NotSpecialAccount = 94,
    ///*
    /// Negative gas was offered in smart contract call
    ContractNegativeGas = 95,
    ///*
    /// Negative value / initial balance was specified in a smart contract call / create
    ContractNegativeValue = 96,
    ///*
    /// Failed to update fee file
    InvalidFeeFile = 97,
    ///*
    /// Failed to update exchange rate file
    InvalidExchangeRateFile = 98,
    ///*
    /// Payment tendered for contract local call cannot cover both the fee and the gas
    InsufficientLocalCallGas = 99,
    ///*
    /// Entities with Entity ID below 1000 are not allowed to be deleted
    EntityNotAllowedToDelete = 100,
    ///*
    /// Violating one of these rules: 1) treasury account can update all entities below 0.0.1000, 2)
    /// account 0.0.50 can update all entities from 0.0.51 - 0.0.80, 3) Network Function Master Account
    /// A/c 0.0.50 - Update all Network Function accounts & perform all the Network Functions listed
    /// below, 4) Network Function Accounts: i) A/c 0.0.55 - Update Address Book files (0.0.101/102),
    /// ii) A/c 0.0.56 - Update Fee schedule (0.0.111), iii) A/c 0.0.57 - Update Exchange Rate
    /// (0.0.112).
    AuthorizationFailed = 101,
    ///*
    /// Fee Schedule Proto uploaded but not valid (append or update is required)
    FileUploadedProtoInvalid = 102,
    ///*
    /// Fee Schedule Proto uploaded but not valid (append or update is required)
    FileUploadedProtoNotSavedToDisk = 103,
    ///*
    /// Fee Schedule Proto File Part uploaded
    FeeScheduleFilePartUploaded = 104,
    ///*
    /// The change on Exchange Rate exceeds Exchange_Rate_Allowed_Percentage
    ExchangeRateChangeLimitExceeded = 105,
    ///*
    /// Contract permanent storage exceeded the currently allowable limit
    MaxContractStorageExceeded = 106,
    ///*
    /// Transfer Account should not be same as Account to be deleted
    TransferAccountSameAsDeleteAccount = 107,
    TotalLedgerBalanceInvalid = 108,
    ///*
    /// The expiration date/time on a smart contract may not be reduced
    ExpirationReductionNotAllowed = 110,
    ///*
    /// Gas exceeded currently allowable gas limit per transaction
    MaxGasLimitExceeded = 111,
    ///*
    /// File size exceeded the currently allowable limit
    MaxFileSizeExceeded = 112,
    ///*
    /// When a valid signature is not provided for operations on account with receiverSigRequired=true
    ReceiverSigRequired = 113,
    ///*
    /// The Topic ID specified is not in the system.
    InvalidTopicId = 150,
    ///*
    /// A provided admin key was invalid.
    InvalidAdminKey = 155,
    ///*
    /// A provided submit key was invalid.
    InvalidSubmitKey = 156,
    ///*
    /// An attempted operation was not authorized (ie - a deleteTopic for a topic with no adminKey).
    Unauthorized = 157,
    ///*
    /// A ConsensusService message is empty.
    InvalidTopicMessage = 158,
    ///*
    /// The autoRenewAccount specified is not a valid, active account.
    InvalidAutorenewAccount = 159,
    ///*
    /// An adminKey was not specified on the topic, so there must not be an autoRenewAccount.
    AutorenewAccountNotAllowed = 160,
    ///*
    /// The topic has expired, was not automatically renewed, and is in a 7 day grace period before the
    /// topic will be deleted unrecoverably. This error response code will not be returned until
    /// autoRenew functionality is supported by HAPI.
    TopicExpired = 162,
    /// chunk number must be from 1 to total (chunks) inclusive.
    InvalidChunkNumber = 163,
    /// For every chunk, the payer account that is part of initialTransactionID must match the Payer Account of this transaction. The entire initialTransactionID should match the transactionID of the first chunk, but this is not checked or enforced by Hedera except when the chunk number is 1.
    InvalidChunkTransactionId = 164,
    /// Account is frozen and cannot transact with the token
    AccountFrozenForToken = 165,
    /// An involved account already has more than <tt>tokens.maxPerAccount</tt> associations with non-deleted tokens.
    TokensPerAccountLimitExceeded = 166,
    /// The token is invalid or does not exist
    InvalidTokenId = 167,
    /// Invalid token decimals
    InvalidTokenDecimals = 168,
    /// Invalid token initial supply
    InvalidTokenInitialSupply = 169,
    /// Treasury Account does not exist or is deleted
    InvalidTreasuryAccountForToken = 170,
    /// Token Symbol is not UTF-8 capitalized alphabetical string
    InvalidTokenSymbol = 171,
    /// Freeze key is not set on token
    TokenHasNoFreezeKey = 172,
    /// Amounts in transfer list are not net zero
    TransfersNotZeroSumForToken = 173,
    /// A token symbol was not provided
    MissingTokenSymbol = 174,
    /// The provided token symbol was too long
    TokenSymbolTooLong = 175,
    /// KYC must be granted and account does not have KYC granted
    AccountKycNotGrantedForToken = 176,
    /// KYC key is not set on token
    TokenHasNoKycKey = 177,
    /// Token balance is not sufficient for the transaction
    InsufficientTokenBalance = 178,
    /// Token transactions cannot be executed on deleted token
    TokenWasDeleted = 179,
    /// Supply key is not set on token
    TokenHasNoSupplyKey = 180,
    /// Wipe key is not set on token
    TokenHasNoWipeKey = 181,
    /// The requested token mint amount would cause an invalid total supply
    InvalidTokenMintAmount = 182,
    /// The requested token burn amount would cause an invalid total supply
    InvalidTokenBurnAmount = 183,
    /// A required token-account relationship is missing
    TokenNotAssociatedToAccount = 184,
    /// The target of a wipe operation was the token treasury account
    CannotWipeTokenTreasuryAccount = 185,
    /// The provided KYC key was invalid.
    InvalidKycKey = 186,
    /// The provided wipe key was invalid.
    InvalidWipeKey = 187,
    /// The provided freeze key was invalid.
    InvalidFreezeKey = 188,
    /// The provided supply key was invalid.
    InvalidSupplyKey = 189,
    /// Token Name is not provided
    MissingTokenName = 190,
    /// Token Name is too long
    TokenNameTooLong = 191,
    /// The provided wipe amount must not be negative, zero or bigger than the token holder balance
    InvalidWipingAmount = 192,
    /// Token does not have Admin key set, thus update/delete transactions cannot be performed
    TokenIsImmutable = 193,
    /// An <tt>associateToken</tt> operation specified a token already associated to the account
    TokenAlreadyAssociatedToAccount = 194,
    /// An attempted operation is invalid until all token balances for the target account are zero
    TransactionRequiresZeroTokenBalances = 195,
    /// An attempted operation is invalid because the account is a treasury
    AccountIsTreasury = 196,
    /// Same TokenIDs present in the token list
    TokenIdRepeatedInTokenList = 197,
    /// Exceeded the number of token transfers (both from and to) allowed for token transfer list
    TokenTransferListSizeLimitExceeded = 198,
    /// TokenTransfersTransactionBody has no TokenTransferList
    EmptyTokenTransferBody = 199,
    /// TokenTransfersTransactionBody has a TokenTransferList with no AccountAmounts
    EmptyTokenTransferAccountAmounts = 200,
    ///*
    /// The Scheduled entity does not exist; or has now expired, been deleted, or been executed
    InvalidScheduleId = 201,
    ///*
    /// The Scheduled entity cannot be modified. Admin key not set
    ScheduleIsImmutable = 202,
    ///*
    /// The provided Scheduled Payer does not exist
    InvalidSchedulePayerId = 203,
    ///*
    /// The Schedule Create Transaction TransactionID account does not exist
    InvalidScheduleAccountId = 204,
    ///*
    /// The provided sig map did not contain any new valid signatures from required signers of the scheduled transaction
    NoNewValidSignatures = 205,
    ///*
    /// The required signers for a scheduled transaction cannot be resolved, for example because they do not exist or have been deleted
    UnresolvableRequiredSigners = 206,
    ///*
    /// Only whitelisted transaction types may be scheduled
    ScheduledTransactionNotInWhitelist = 207,
    ///*
    /// At least one of the signatures in the provided sig map did not represent a valid signature for any required signer
    SomeSignaturesWereInvalid = 208,
    ///*
    /// The scheduled field in the TransactionID may not be set to true
    TransactionIdFieldNotAllowed = 209,
    ///*
    /// A schedule already exists with the same identifying fields of an attempted ScheduleCreate (that is, all fields other than scheduledPayerAccountID)
    IdenticalScheduleAlreadyCreated = 210,
    ///*
    /// A string field in the transaction has a UTF-8 encoding with the prohibited zero byte
    InvalidZeroByteInString = 211,
    ///*
    /// A schedule being signed or deleted has already been deleted
    ScheduleAlreadyDeleted = 212,
    ///*
    /// A schedule being signed or deleted has already been executed
    ScheduleAlreadyExecuted = 213,
    ///*
    /// ConsensusSubmitMessage request's message size is larger than allowed.
    MessageSizeTooLarge = 214,
    ///*
    /// An operation was assigned to more than one throttle group in a given bucket
    OperationRepeatedInBucketGroups = 215,
    ///*
    /// The capacity needed to satisfy all opsPerSec groups in a bucket overflowed a signed 8-byte integral type
    BucketCapacityOverflow = 216,
    ///*
    /// Given the network size in the address book, the node-level capacity for an operation would never be enough to accept a single request; usually means a bucket burstPeriod should be increased
    NodeCapacityNotSufficientForOperation = 217,
    ///*
    /// A bucket was defined without any throttle groups
    BucketHasNoThrottleGroups = 218,
    ///*
    /// A throttle group was granted zero opsPerSec
    ThrottleGroupHasZeroOpsPerSec = 219,
    ///*
    /// The throttle definitions file was updated, but some supported operations were not assigned a bucket
    SuccessButMissingExpectedOperation = 220,
    ///*
    /// The new contents for the throttle definitions system file were not valid protobuf
    UnparseableThrottleDefinitions = 221,
    ///*
    /// The new throttle definitions system file were invalid, and no more specific error could be divined
    InvalidThrottleDefinitions = 222,
    ///*
    /// The transaction references an account which has passed its expiration without renewal funds available, and currently remains in the ledger only because of the grace period given to expired entities
    AccountExpiredAndPendingRemoval = 223,
    ///*
    /// Invalid token max supply
    InvalidTokenMaxSupply = 224,
    ///*
    /// Invalid token nft serial number
    InvalidTokenNftSerialNumber = 225,
    ///*
    /// Invalid nft id
    InvalidNftId = 226,
    ///*
    /// Nft metadata is too long
    MetadataTooLong = 227,
    ///*
    /// Repeated operations count exceeds the limit
    BatchSizeLimitExceeded = 228,
    ///*
    /// The range of data to be gathered is out of the set boundaries
    InvalidQueryRange = 229,
    ///*
    /// A custom fractional fee set a denominator of zero
    FractionDividesByZero = 230,
    ///*
    /// The transaction payer could not afford a custom fee
    InsufficientPayerBalanceForCustomFee = 231,
    ///*
    /// More than 10 custom fees were specified
    CustomFeesListTooLong = 232,
    ///*
    /// Any of the feeCollector accounts for customFees is invalid
    InvalidCustomFeeCollector = 233,
    ///*
    /// Any of the token Ids in customFees is invalid
    InvalidTokenIdInCustomFees = 234,
    ///*
    /// Any of the token Ids in customFees are not associated to feeCollector
    TokenNotAssociatedToFeeCollector = 235,
    ///*
    /// A token cannot have more units minted due to its configured supply ceiling
    TokenMaxSupplyReached = 236,
    ///*
    /// The transaction attempted to move an NFT serial number from an account other than its owner
    SenderDoesNotOwnNftSerialNo = 237,
    ///*
    /// A custom fee schedule entry did not specify either a fixed or fractional fee
    CustomFeeNotFullySpecified = 238,
    ///*
    /// Only positive fees may be assessed at this time
    CustomFeeMustBePositive = 239,
    ///*
    /// Fee schedule key is not set on token
    TokenHasNoFeeScheduleKey = 240,
    ///*
    /// A fractional custom fee exceeded the range of a 64-bit signed integer
    CustomFeeOutsideNumericRange = 241,
    ///*
    /// A royalty cannot exceed the total fungible value exchanged for an NFT
    RoyaltyFractionCannotExceedOne = 242,
    ///*
    /// Each fractional custom fee must have its maximum_amount, if specified, at least its minimum_amount
    FractionalFeeMaxAmountLessThanMinAmount = 243,
    ///*
    /// A fee schedule update tried to clear the custom fees from a token whose fee schedule was already empty
    CustomScheduleAlreadyHasNoFees = 244,
    ///*
    /// Only tokens of type FUNGIBLE_COMMON can be used to as fee schedule denominations
    CustomFeeDenominationMustBeFungibleCommon = 245,
    ///*
    /// Only tokens of type FUNGIBLE_COMMON can have fractional fees
    CustomFractionalFeeOnlyAllowedForFungibleCommon = 246,
    ///*
    /// The provided custom fee schedule key was invalid
    InvalidCustomFeeScheduleKey = 247,
    ///*
    /// The requested token mint metadata was invalid
    InvalidTokenMintMetadata = 248,
    ///*
    /// The requested token burn metadata was invalid
    InvalidTokenBurnMetadata = 249,
    ///*
    /// The treasury for a unique token cannot be changed until it owns no NFTs
    CurrentTreasuryStillOwnsNfts = 250,
    ///*
    /// An account cannot be dissociated from a unique token if it owns NFTs for the token
    AccountStillOwnsNfts = 251,
    ///*
    /// A NFT can only be burned when owned by the unique token's treasury
    TreasuryMustOwnBurnedNft = 252,
    ///*
    /// An account did not own the NFT to be wiped
    AccountDoesNotOwnWipedNft = 253,
    ///*
    /// An AccountAmount token transfers list referenced a token type other than FUNGIBLE_COMMON
    AccountAmountTransfersOnlyAllowedForFungibleCommon = 254,
    ///*
    /// All the NFTs allowed in the current price regime have already been minted
    MaxNftsInPriceRegimeHaveBeenMinted = 255,
    ///*
    /// The payer account has been marked as deleted
    PayerAccountDeleted = 256,
    ///*
    /// The reference chain of custom fees for a transferred token exceeded the maximum length of 2
    CustomFeeChargingExceededMaxRecursionDepth = 257,
    ///*
    /// More than 20 balance adjustments were to satisfy a CryptoTransfer and its implied custom fee payments
    CustomFeeChargingExceededMaxAccountAmounts = 258,
    ///*
    /// The sender account in the token transfer transaction could not afford a custom fee
    InsufficientSenderAccountBalanceForCustomFee = 259,
    ///*
    /// Currently no more than 4,294,967,295 NFTs may be minted for a given unique token type
    SerialNumberLimitReached = 260,
    ///*
    /// Only tokens of type NON_FUNGIBLE_UNIQUE can have royalty fees
    CustomRoyaltyFeeOnlyAllowedForNonFungibleUnique = 261,
    ///*
    /// The account has reached the limit on the automatic associations count.
    NoRemainingAutomaticAssociations = 262,
    ///*
    /// Already existing automatic associations are more than the new maximum automatic associations.
    ExistingAutomaticAssociationsExceedGivenLimit = 263,
    ///*
    /// Cannot set the number of automatic associations for an account more than the maximum allowed
    /// token associations <tt>tokens.maxPerAccount</tt>.
    RequestedNumAutomaticAssociationsExceedsAssociationLimit = 264,
    ///*
    /// Token is paused. This Token cannot be a part of any kind of Transaction until unpaused.
    TokenIsPaused = 265,
    ///*
    /// Pause key is not set on token
    TokenHasNoPauseKey = 266,
    ///*
    /// The provided pause key was invalid
    InvalidPauseKey = 267,
    ///*
    /// The update file in a freeze transaction body must exist.
    FreezeUpdateFileDoesNotExist = 268,
    ///*
    /// The hash of the update file in a freeze transaction body must match the in-memory hash.
    FreezeUpdateFileHashDoesNotMatch = 269,
    ///*
    /// A FREEZE_UPGRADE transaction was handled with no previous update prepared.
    NoUpgradeHasBeenPrepared = 270,
    ///*
    /// A FREEZE_ABORT transaction was handled with no scheduled freeze.
    NoFreezeIsScheduled = 271,
    ///*
    /// The update file hash when handling a FREEZE_UPGRADE transaction differs from the file
    /// hash at the time of handling the PREPARE_UPGRADE transaction.
    UpdateFileHashChangedSincePrepareUpgrade = 272,
    ///*
    /// The given freeze start time was in the (consensus) past.
    FreezeStartTimeMustBeFuture = 273,
    ///*
    /// The prepared update file cannot be updated or appended until either the upgrade has
    /// been completed, or a FREEZE_ABORT has been handled.
    PreparedUpdateFileIsImmutable = 274,
    ///*
    /// Once a freeze is scheduled, it must be aborted before any other type of freeze can
    /// can be performed.
    FreezeAlreadyScheduled = 275,
    ///*
    /// If an NMT upgrade has been prepared, the following operation must be a FREEZE_UPGRADE.
    /// (To issue a FREEZE_ONLY, submit a FREEZE_ABORT first.)
    FreezeUpgradeInProgress = 276,
    ///*
    /// If an NMT upgrade has been prepared, the subsequent FREEZE_UPGRADE transaction must
    /// confirm the id of the file to be used in the upgrade.
    UpdateFileIdDoesNotMatchPrepared = 277,
    ///*
    /// If an NMT upgrade has been prepared, the subsequent FREEZE_UPGRADE transaction must
    /// confirm the hash of the file to be used in the upgrade.
    UpdateFileHashDoesNotMatchPrepared = 278,
    ///*
    /// Consensus throttle did not allow execution of this transaction. System is throttled at
    /// consensus level.
    ConsensusGasExhausted = 279,
    ///*
    /// A precompiled contract succeeded, but was later reverted.
    RevertedSuccess = 280,
    ///*
    /// All contract storage allocated to the current price regime has been consumed.
    MaxStorageInPriceRegimeHasBeenUsed = 281,
    ///*
    /// An alias used in a CryptoTransfer transaction is not the serialization of a primitive Key
    /// message--that is, a Key with a single Ed25519 or ECDSA(secp256k1) public key and no
    /// unknown protobuf fields.
    InvalidAliasKey = 282,
    ///*
    /// A fungible token transfer expected a different number of decimals than the involved
    /// type actually has.
    UnexpectedTokenDecimals = 283,
}
