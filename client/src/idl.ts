export type Cronos = {
  "version": "0.0.16",
  "name": "cronos",
  "instructions": [
    {
      "name": "adminCancelTask",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "config",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "daemon",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "task",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "adminCreateTask",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "clock",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "config",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "daemon",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "task",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "ix",
          "type": {
            "defined": "InstructionData"
          }
        },
        {
          "name": "execAt",
          "type": "i64"
        },
        {
          "name": "stopAt",
          "type": "i64"
        },
        {
          "name": "recurr",
          "type": "i64"
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "adminResetHealth",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "clock",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "config",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "health",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "configUpdateAdmin",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "config",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "newAdmin",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "configUpdateProgramFee",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "config",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "newProgramFee",
          "type": "u64"
        }
      ]
    },
    {
      "name": "configUpdateWorkerFee",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "config",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "newWorkerFee",
          "type": "u64"
        }
      ]
    },
    {
      "name": "daemonCreate",
      "accounts": [
        {
          "name": "daemon",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fee",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "daemonBump",
          "type": "u8"
        },
        {
          "name": "feeBump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "daemonInvoke",
      "accounts": [
        {
          "name": "daemon",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "instructionData",
          "type": {
            "defined": "InstructionData"
          }
        }
      ]
    },
    {
      "name": "feeCollect",
      "accounts": [
        {
          "name": "fee",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "config",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "daemon",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fee",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "health",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "authorityBump",
          "type": "u8"
        },
        {
          "name": "configBump",
          "type": "u8"
        },
        {
          "name": "daemonBump",
          "type": "u8"
        },
        {
          "name": "feeBump",
          "type": "u8"
        },
        {
          "name": "healthBump",
          "type": "u8"
        },
        {
          "name": "treasuryBump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "healthCheck",
      "accounts": [
        {
          "name": "clock",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "daemon",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "health",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "taskCancel",
      "accounts": [
        {
          "name": "daemon",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "task",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "taskCreate",
      "accounts": [
        {
          "name": "clock",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "config",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "daemon",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "task",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "ix",
          "type": {
            "defined": "InstructionData"
          }
        },
        {
          "name": "execAt",
          "type": "i64"
        },
        {
          "name": "stopAt",
          "type": "i64"
        },
        {
          "name": "recurr",
          "type": "i64"
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "taskExecute",
      "accounts": [
        {
          "name": "clock",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "config",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "daemon",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fee",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "task",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "worker",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "authority",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "config",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "type": "publicKey"
          },
          {
            "name": "minRecurr",
            "type": "i64"
          },
          {
            "name": "programFee",
            "type": "u64"
          },
          {
            "name": "workerFee",
            "type": "u64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "daemon",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "publicKey"
          },
          {
            "name": "taskCount",
            "type": "u128"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "fee",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "daemon",
            "type": "publicKey"
          },
          {
            "name": "balance",
            "type": "u64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "health",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "realTime",
            "type": "i64"
          },
          {
            "name": "targetTime",
            "type": "i64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "task",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "daemon",
            "type": "publicKey"
          },
          {
            "name": "id",
            "type": "u128"
          },
          {
            "name": "ix",
            "type": {
              "defined": "InstructionData"
            }
          },
          {
            "name": "status",
            "type": {
              "defined": "TaskStatus"
            }
          },
          {
            "name": "execAt",
            "type": "i64"
          },
          {
            "name": "stopAt",
            "type": "i64"
          },
          {
            "name": "recurr",
            "type": "i64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "treasury",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "InstructionData",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "programId",
            "type": "publicKey"
          },
          {
            "name": "accounts",
            "type": {
              "vec": {
                "defined": "AccountMetaData"
              }
            }
          },
          {
            "name": "data",
            "type": "bytes"
          }
        ]
      }
    },
    {
      "name": "AccountMetaData",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pubkey",
            "type": "publicKey"
          },
          {
            "name": "isSigner",
            "type": "bool"
          },
          {
            "name": "isWritable",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "TaskStatus",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Cancelled"
          },
          {
            "name": "Executed"
          },
          {
            "name": "Pending"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidChronology",
      "msg": "Tasks cannot be started before they are stopped"
    },
    {
      "code": 6001,
      "name": "InvalidExecAtStale",
      "msg": "Tasks cannot be scheduled for execution in the past"
    },
    {
      "code": 6002,
      "name": "InvalidRecurrNegative",
      "msg": "Recurrence interval cannot be negative"
    },
    {
      "code": 6003,
      "name": "InvalidRecurrBelowMin",
      "msg": "Recurrence interval is below the minimum supported time granulartiy"
    },
    {
      "code": 6004,
      "name": "InvalidSignatory",
      "msg": "Your daemon cannot provide all required signatures for this instruction"
    },
    {
      "code": 6005,
      "name": "TaskNotPending",
      "msg": "Task is not pending and may not executed"
    },
    {
      "code": 6006,
      "name": "TaskNotDue",
      "msg": "This task is not due and may not be executed yet"
    },
    {
      "code": 6007,
      "name": "Unknown",
      "msg": "Unknown error"
    }
  ]
};

export const IDL: Cronos = {
  "version": "0.0.16",
  "name": "cronos",
  "instructions": [
    {
      "name": "adminCancelTask",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "config",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "daemon",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "task",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "adminCreateTask",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "clock",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "config",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "daemon",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "task",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "ix",
          "type": {
            "defined": "InstructionData"
          }
        },
        {
          "name": "execAt",
          "type": "i64"
        },
        {
          "name": "stopAt",
          "type": "i64"
        },
        {
          "name": "recurr",
          "type": "i64"
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "adminResetHealth",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "clock",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "config",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "health",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "configUpdateAdmin",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "config",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "newAdmin",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "configUpdateProgramFee",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "config",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "newProgramFee",
          "type": "u64"
        }
      ]
    },
    {
      "name": "configUpdateWorkerFee",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "config",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "newWorkerFee",
          "type": "u64"
        }
      ]
    },
    {
      "name": "daemonCreate",
      "accounts": [
        {
          "name": "daemon",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fee",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "daemonBump",
          "type": "u8"
        },
        {
          "name": "feeBump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "daemonInvoke",
      "accounts": [
        {
          "name": "daemon",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "instructionData",
          "type": {
            "defined": "InstructionData"
          }
        }
      ]
    },
    {
      "name": "feeCollect",
      "accounts": [
        {
          "name": "fee",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "config",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "daemon",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fee",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "health",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "authorityBump",
          "type": "u8"
        },
        {
          "name": "configBump",
          "type": "u8"
        },
        {
          "name": "daemonBump",
          "type": "u8"
        },
        {
          "name": "feeBump",
          "type": "u8"
        },
        {
          "name": "healthBump",
          "type": "u8"
        },
        {
          "name": "treasuryBump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "healthCheck",
      "accounts": [
        {
          "name": "clock",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "daemon",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "health",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "taskCancel",
      "accounts": [
        {
          "name": "daemon",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "task",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "taskCreate",
      "accounts": [
        {
          "name": "clock",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "config",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "daemon",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "task",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "ix",
          "type": {
            "defined": "InstructionData"
          }
        },
        {
          "name": "execAt",
          "type": "i64"
        },
        {
          "name": "stopAt",
          "type": "i64"
        },
        {
          "name": "recurr",
          "type": "i64"
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "taskExecute",
      "accounts": [
        {
          "name": "clock",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "config",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "daemon",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fee",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "task",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "worker",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "authority",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "config",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "type": "publicKey"
          },
          {
            "name": "minRecurr",
            "type": "i64"
          },
          {
            "name": "programFee",
            "type": "u64"
          },
          {
            "name": "workerFee",
            "type": "u64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "daemon",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "publicKey"
          },
          {
            "name": "taskCount",
            "type": "u128"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "fee",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "daemon",
            "type": "publicKey"
          },
          {
            "name": "balance",
            "type": "u64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "health",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "realTime",
            "type": "i64"
          },
          {
            "name": "targetTime",
            "type": "i64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "task",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "daemon",
            "type": "publicKey"
          },
          {
            "name": "id",
            "type": "u128"
          },
          {
            "name": "ix",
            "type": {
              "defined": "InstructionData"
            }
          },
          {
            "name": "status",
            "type": {
              "defined": "TaskStatus"
            }
          },
          {
            "name": "execAt",
            "type": "i64"
          },
          {
            "name": "stopAt",
            "type": "i64"
          },
          {
            "name": "recurr",
            "type": "i64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "treasury",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "InstructionData",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "programId",
            "type": "publicKey"
          },
          {
            "name": "accounts",
            "type": {
              "vec": {
                "defined": "AccountMetaData"
              }
            }
          },
          {
            "name": "data",
            "type": "bytes"
          }
        ]
      }
    },
    {
      "name": "AccountMetaData",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pubkey",
            "type": "publicKey"
          },
          {
            "name": "isSigner",
            "type": "bool"
          },
          {
            "name": "isWritable",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "TaskStatus",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Cancelled"
          },
          {
            "name": "Executed"
          },
          {
            "name": "Pending"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidChronology",
      "msg": "Tasks cannot be started before they are stopped"
    },
    {
      "code": 6001,
      "name": "InvalidExecAtStale",
      "msg": "Tasks cannot be scheduled for execution in the past"
    },
    {
      "code": 6002,
      "name": "InvalidRecurrNegative",
      "msg": "Recurrence interval cannot be negative"
    },
    {
      "code": 6003,
      "name": "InvalidRecurrBelowMin",
      "msg": "Recurrence interval is below the minimum supported time granulartiy"
    },
    {
      "code": 6004,
      "name": "InvalidSignatory",
      "msg": "Your daemon cannot provide all required signatures for this instruction"
    },
    {
      "code": 6005,
      "name": "TaskNotPending",
      "msg": "Task is not pending and may not executed"
    },
    {
      "code": 6006,
      "name": "TaskNotDue",
      "msg": "This task is not due and may not be executed yet"
    },
    {
      "code": 6007,
      "name": "Unknown",
      "msg": "Unknown error"
    }
  ]
};
