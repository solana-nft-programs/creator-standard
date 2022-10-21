export type CardinalCreatorStandard = {
  version: "0.1.0";
  name: "cardinal_creator_standard";
  instructions: [
    {
      name: "initMintManager";
      accounts: [
        {
          name: "mintManager";
          isMut: true;
          isSigner: false;
        },
        {
          name: "mint";
          isMut: true;
          isSigner: false;
        },
        {
          name: "standard";
          isMut: false;
          isSigner: false;
        },
        {
          name: "collector";
          isMut: true;
          isSigner: false;
        },
        {
          name: "authority";
          isMut: false;
          isSigner: true;
        },
        {
          name: "payer";
          isMut: true;
          isSigner: true;
        },
        {
          name: "tokenProgram";
          isMut: false;
          isSigner: false;
        },
        {
          name: "systemProgram";
          isMut: false;
          isSigner: false;
        }
      ];
      args: [];
    },
    {
      name: "updateMintManager";
      accounts: [
        {
          name: "mintManager";
          isMut: false;
          isSigner: false;
        },
        {
          name: "standard";
          isMut: false;
          isSigner: false;
        },
        {
          name: "authority";
          isMut: false;
          isSigner: true;
        },
        {
          name: "systemProgram";
          isMut: false;
          isSigner: false;
        }
      ];
      args: [
        {
          name: "ix";
          type: {
            defined: "UpdateMintManagerIx";
          };
        }
      ];
    },
    {
      name: "initStandard";
      accounts: [
        {
          name: "standard";
          isMut: true;
          isSigner: false;
        },
        {
          name: "authority";
          isMut: false;
          isSigner: true;
        },
        {
          name: "payer";
          isMut: true;
          isSigner: true;
        },
        {
          name: "systemProgram";
          isMut: false;
          isSigner: false;
        }
      ];
      args: [
        {
          name: "ix";
          type: {
            defined: "InitStandardIx";
          };
        }
      ];
    },
    {
      name: "updateStandard";
      accounts: [
        {
          name: "standard";
          isMut: true;
          isSigner: false;
        },
        {
          name: "authority";
          isMut: false;
          isSigner: true;
        },
        {
          name: "systemProgram";
          isMut: false;
          isSigner: false;
        }
      ];
      args: [
        {
          name: "ix";
          type: {
            defined: "UpdateStandardIx";
          };
        }
      ];
    }
  ];
  accounts: [
    {
      name: "mintManager";
      type: {
        kind: "struct";
        fields: [
          {
            name: "bump";
            type: "u8";
          },
          {
            name: "version";
            type: "u8";
          },
          {
            name: "mint";
            type: "publicKey";
          },
          {
            name: "authority";
            type: "publicKey";
          },
          {
            name: "standard";
            type: "publicKey";
          }
        ];
      };
    },
    {
      name: "standard";
      type: {
        kind: "struct";
        fields: [
          {
            name: "bump";
            type: "u8";
          },
          {
            name: "version";
            type: "u8";
          },
          {
            name: "authority";
            type: "publicKey";
          },
          {
            name: "checkSellerFeeBasisPoints";
            type: "bool";
          },
          {
            name: "name";
            type: "string";
          },
          {
            name: "disallowedPrograms";
            type: {
              vec: "publicKey";
            };
          },
          {
            name: "allowedPrograms";
            type: {
              vec: "publicKey";
            };
          }
        ];
      };
    }
  ];
  types: [
    {
      name: "UpdateMintManagerIx";
      type: {
        kind: "struct";
        fields: [
          {
            name: "authority";
            type: "publicKey";
          }
        ];
      };
    },
    {
      name: "InitStandardIx";
      type: {
        kind: "struct";
        fields: [
          {
            name: "checkSellerFeeBasisPoints";
            type: "bool";
          },
          {
            name: "name";
            type: "string";
          },
          {
            name: "disallowedPrograms";
            type: {
              vec: "publicKey";
            };
          },
          {
            name: "allowedPrograms";
            type: {
              vec: "publicKey";
            };
          }
        ];
      };
    },
    {
      name: "UpdateStandardIx";
      type: {
        kind: "struct";
        fields: [
          {
            name: "checkSellerFeeBasisPoints";
            type: "bool";
          },
          {
            name: "disallowedPrograms";
            type: {
              vec: "publicKey";
            };
          },
          {
            name: "allowedPrograms";
            type: {
              vec: "publicKey";
            };
          }
        ];
      };
    }
  ];
  errors: [
    {
      code: 6000;
      name: "InvalidMint";
      msg: "Invalid mint";
    },
    {
      code: 6001;
      name: "InvalidCollector";
      msg: "Invalid collector address";
    },
    {
      code: 6002;
      name: "InvalidAuthority";
      msg: "Invalid authority address";
    },
    {
      code: 6003;
      name: "InvaldiMint";
      msg: "Invalid mint";
    },
    {
      code: 6004;
      name: "InvlaidHolderTokenAccount";
      msg: "Invalid holder token account";
    },
    {
      code: 6005;
      name: "InvalidTargetTokenAccount";
      msg: "Invalid target token account";
    },
    {
      code: 6006;
      name: "InvalidCloseTokenAccount";
      msg: "Invalid token account to close";
    },
    {
      code: 6007;
      name: "InvalidHolderTokenAccount";
      msg: "Invalid holder token account";
    }
  ];
};

export const IDL: CardinalCreatorStandard = {
  version: "0.1.0",
  name: "cardinal_creator_standard",
  instructions: [
    {
      name: "initMintManager",
      accounts: [
        {
          name: "mintManager",
          isMut: true,
          isSigner: false,
        },
        {
          name: "mint",
          isMut: true,
          isSigner: false,
        },
        {
          name: "standard",
          isMut: false,
          isSigner: false,
        },
        {
          name: "collector",
          isMut: true,
          isSigner: false,
        },
        {
          name: "authority",
          isMut: false,
          isSigner: true,
        },
        {
          name: "payer",
          isMut: true,
          isSigner: true,
        },
        {
          name: "tokenProgram",
          isMut: false,
          isSigner: false,
        },
        {
          name: "systemProgram",
          isMut: false,
          isSigner: false,
        },
      ],
      args: [],
    },
    {
      name: "updateMintManager",
      accounts: [
        {
          name: "mintManager",
          isMut: false,
          isSigner: false,
        },
        {
          name: "standard",
          isMut: false,
          isSigner: false,
        },
        {
          name: "authority",
          isMut: false,
          isSigner: true,
        },
        {
          name: "systemProgram",
          isMut: false,
          isSigner: false,
        },
      ],
      args: [
        {
          name: "ix",
          type: {
            defined: "UpdateMintManagerIx",
          },
        },
      ],
    },
    {
      name: "initStandard",
      accounts: [
        {
          name: "standard",
          isMut: true,
          isSigner: false,
        },
        {
          name: "authority",
          isMut: false,
          isSigner: true,
        },
        {
          name: "payer",
          isMut: true,
          isSigner: true,
        },
        {
          name: "systemProgram",
          isMut: false,
          isSigner: false,
        },
      ],
      args: [
        {
          name: "ix",
          type: {
            defined: "InitStandardIx",
          },
        },
      ],
    },
    {
      name: "updateStandard",
      accounts: [
        {
          name: "standard",
          isMut: true,
          isSigner: false,
        },
        {
          name: "authority",
          isMut: false,
          isSigner: true,
        },
        {
          name: "systemProgram",
          isMut: false,
          isSigner: false,
        },
      ],
      args: [
        {
          name: "ix",
          type: {
            defined: "UpdateStandardIx",
          },
        },
      ],
    },
  ],
  accounts: [
    {
      name: "mintManager",
      type: {
        kind: "struct",
        fields: [
          {
            name: "bump",
            type: "u8",
          },
          {
            name: "version",
            type: "u8",
          },
          {
            name: "mint",
            type: "publicKey",
          },
          {
            name: "authority",
            type: "publicKey",
          },
          {
            name: "standard",
            type: "publicKey",
          },
        ],
      },
    },
    {
      name: "standard",
      type: {
        kind: "struct",
        fields: [
          {
            name: "bump",
            type: "u8",
          },
          {
            name: "version",
            type: "u8",
          },
          {
            name: "authority",
            type: "publicKey",
          },
          {
            name: "checkSellerFeeBasisPoints",
            type: "bool",
          },
          {
            name: "name",
            type: "string",
          },
          {
            name: "disallowedPrograms",
            type: {
              vec: "publicKey",
            },
          },
          {
            name: "allowedPrograms",
            type: {
              vec: "publicKey",
            },
          },
        ],
      },
    },
  ],
  types: [
    {
      name: "UpdateMintManagerIx",
      type: {
        kind: "struct",
        fields: [
          {
            name: "authority",
            type: "publicKey",
          },
        ],
      },
    },
    {
      name: "InitStandardIx",
      type: {
        kind: "struct",
        fields: [
          {
            name: "checkSellerFeeBasisPoints",
            type: "bool",
          },
          {
            name: "name",
            type: "string",
          },
          {
            name: "disallowedPrograms",
            type: {
              vec: "publicKey",
            },
          },
          {
            name: "allowedPrograms",
            type: {
              vec: "publicKey",
            },
          },
        ],
      },
    },
    {
      name: "UpdateStandardIx",
      type: {
        kind: "struct",
        fields: [
          {
            name: "checkSellerFeeBasisPoints",
            type: "bool",
          },
          {
            name: "disallowedPrograms",
            type: {
              vec: "publicKey",
            },
          },
          {
            name: "allowedPrograms",
            type: {
              vec: "publicKey",
            },
          },
        ],
      },
    },
  ],
  errors: [
    {
      code: 6000,
      name: "InvalidMint",
      msg: "Invalid mint",
    },
    {
      code: 6001,
      name: "InvalidCollector",
      msg: "Invalid collector address",
    },
    {
      code: 6002,
      name: "InvalidAuthority",
      msg: "Invalid authority address",
    },
    {
      code: 6003,
      name: "InvaldiMint",
      msg: "Invalid mint",
    },
    {
      code: 6004,
      name: "InvlaidHolderTokenAccount",
      msg: "Invalid holder token account",
    },
    {
      code: 6005,
      name: "InvalidTargetTokenAccount",
      msg: "Invalid target token account",
    },
    {
      code: 6006,
      name: "InvalidCloseTokenAccount",
      msg: "Invalid token account to close",
    },
    {
      code: 6007,
      name: "InvalidHolderTokenAccount",
      msg: "Invalid holder token account",
    },
  ],
};
