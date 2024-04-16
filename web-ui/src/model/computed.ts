import type { PieceType } from "./model";

export namespace PieceInfo {
  export const SpawnLocation: { [key in PieceType]: [number, number] } = {
    O: [3, 20],
    I: [3, 19],
    T: [3, 20],
    L: [3, 20],
    J: [3, 20],
    S: [3, 20],
    Z: [3, 20],
  };

  export const Shapes: { [key in PieceType]: boolean[][][] } = {
    O: [
      [
        [false, false, false, false],
        [false, true, true, false],
        [false, true, true, false],
        [false, false, false, false],
      ],
      [
        [false, false, false, false],
        [false, true, true, false],
        [false, true, true, false],
        [false, false, false, false],
      ],
      [
        [false, false, false, false],
        [false, true, true, false],
        [false, true, true, false],
        [false, false, false, false],
      ],
      [
        [false, false, false, false],
        [false, true, true, false],
        [false, true, true, false],
        [false, false, false, false],
      ],
    ],
    I: [
      [
        [false, false, true, false],
        [false, false, true, false],
        [false, false, true, false],
        [false, false, true, false],
      ],
      [
        [false, false, false, false],
        [false, false, false, false],
        [true, true, true, true],
        [false, false, false, false],
      ],
      [
        [false, true, false, false],
        [false, true, false, false],
        [false, true, false, false],
        [false, true, false, false],
      ],
      [
        [false, false, false, false],
        [true, true, true, true],
        [false, false, false, false],
        [false, false, false, false],
      ],
    ],
    T: [
      [
        [false, true, false, false],
        [false, true, true, false],
        [false, true, false, false],
        [false, false, false, false],
      ],
      [
        [false, false, false, false],
        [true, true, true, false],
        [false, true, false, false],
        [false, false, false, false],
      ],
      [
        [false, true, false, false],
        [true, true, false, false],
        [false, true, false, false],
        [false, false, false, false],
      ],
      [
        [false, true, false, false],
        [true, true, true, false],
        [false, false, false, false],
        [false, false, false, false],
      ],
    ],
    L: [
      [
        [false, true, false, false],
        [false, true, false, false],
        [false, true, true, false],
        [false, false, false, false],
      ],
      [
        [false, false, false, false],
        [true, true, true, false],
        [true, false, false, false],
        [false, false, false, false],
      ],
      [
        [true, true, false, false],
        [false, true, false, false],
        [false, true, false, false],
        [false, false, false, false],
      ],
      [
        [false, false, true, false],
        [true, true, true, false],
        [false, false, false, false],
        [false, false, false, false],
      ],
    ],
    J: [
      [
        [false, true, true, false],
        [false, true, false, false],
        [false, true, false, false],
        [false, false, false, false],
      ],
      [
        [false, false, false, false],
        [true, true, true, false],
        [false, false, true, false],
        [false, false, false, false],
      ],
      [
        [false, true, false, false],
        [false, true, false, false],
        [true, true, false, false],
        [false, false, false, false],
      ],
      [
        [true, false, false, false],
        [true, true, true, false],
        [false, false, false, false],
        [false, false, false, false],
      ],
    ],
    S: [
      [
        [false, true, false, false],
        [false, true, true, false],
        [false, false, true, false],
        [false, false, false, false],
      ],
      [
        [false, false, false, false],
        [false, true, true, false],
        [true, true, false, false],
        [false, false, false, false],
      ],
      [
        [true, false, false, false],
        [true, true, false, false],
        [false, true, false, false],
        [false, false, false, false],
      ],
      [
        [false, true, true, false],
        [true, true, false, false],
        [false, false, false, false],
        [false, false, false, false],
      ],
    ],
    Z: [
      [
        [false, false, true, false],
        [false, true, true, false],
        [false, true, false, false],
        [false, false, false, false],
      ],
      [
        [false, false, false, false],
        [true, true, false, false],
        [false, true, true, false],
        [false, false, false, false],
      ],
      [
        [false, true, false, false],
        [true, true, false, false],
        [true, false, false, false],
        [false, false, false, false],
      ],
      [
        [true, true, false, false],
        [false, true, true, false],
        [false, false, false, false],
        [false, false, false, false],
      ],
    ],
  };

  export const LocationBounds: {
    // min x, max x, min y, max y
    [key in PieceType]: [number, number, number, number][];
  } = {
    O: [
      [-1, 7, -1, 21],
      [-1, 7, -1, 21],
      [-1, 7, -1, 21],
      [-1, 7, -1, 21],
    ],
    I: [
      [0, 6, -2, 21],
      [-2, 7, 0, 20],
      [0, 6, -1, 22],
      [-1, 8, 0, 20],
    ],
    T: [
      [0, 7, -1, 21],
      [-1, 7, 0, 21],
      [0, 7, 0, 22],
      [0, 8, 0, 21],
    ],
    L: [
      [0, 7, -1, 21],
      [-1, 7, 0, 21],
      [0, 7, 0, 22],
      [0, 8, 0, 21],
    ],
    J: [
      [0, 7, -1, 21],
      [-1, 7, 0, 21],
      [0, 7, 0, 22],
      [0, 8, 0, 21],
    ],
    S: [
      [0, 7, -1, 21],
      [-1, 7, 0, 21],
      [0, 7, 0, 22],
      [0, 8, 0, 21],
    ],
    Z: [
      [0, 7, -1, 21],
      [-1, 7, 0, 21],
      [0, 7, 0, 22],
      [0, 8, 0, 21],
    ],
  };

  export const KickTable: { [key in PieceType]: [number, number][][][] } = {
    O: [
      [[], [[0, 0]], [[0, 0]], [[0, 0]]],
      [[[0, 0]], [], [[0, 0]], [[0, 0]]],
      [[[0, 0]], [[0, 0]], [], [[0, 0]]],
      [[[0, 0]], [[0, 0]], [[0, 0]], []],
    ],
    I: [
      [
        [],
        [
          [0, 0],
          [-2, 0],
          [1, 0],
          [-2, -1],
          [1, 2],
        ],
        [[0, 0]],
        [
          [0, 0],
          [-1, 0],
          [2, 0],
          [-1, 2],
          [2, -1],
        ],
      ],
      [
        [
          [0, 0],
          [2, 0],
          [-1, 0],
          [2, 1],
          [-1, -2],
        ],
        [],
        [
          [0, 0],
          [-1, 0],
          [2, 0],
          [-1, 2],
          [2, -1],
        ],
        [[0, 0]],
      ],
      [
        [[0, 0]],
        [
          [0, 0],
          [1, 0],
          [-2, 0],
          [1, -2],
          [-2, 1],
        ],
        [],
        [
          [0, 0],
          [2, 0],
          [-1, 0],
          [2, 1],
          [-1, -2],
        ],
      ],
      [
        [
          [0, 0],
          [1, 0],
          [-2, 0],
          [1, -2],
          [-2, 1],
        ],
        [[0, 0]],
        [
          [0, 0],
          [-2, 0],
          [1, 0],
          [-2, -1],
          [1, 2],
        ],
        [],
      ],
    ],
    T: [
      [
        [],
        [
          [0, 0],
          [-1, 0],
          [-1, 1],
          [0, -2],
          [-1, -2],
        ],
        [[0, 0]],
        [
          [0, 0],
          [1, 0],
          [1, 1],
          [0, -2],
          [1, -2],
        ],
      ],
      [
        [
          [0, 0],
          [1, 0],
          [1, -1],
          [0, 2],
          [1, 2],
        ],
        [],
        [
          [0, 0],
          [1, 0],
          [1, -1],
          [0, 2],
          [1, 2],
        ],
        [[0, 0]],
      ],
      [
        [[0, 0]],
        [
          [0, 0],
          [-1, 0],
          [-1, 1],
          [0, -2],
          [-1, -2],
        ],
        [],
        [
          [0, 0],
          [1, 0],
          [1, 1],
          [0, -2],
          [1, -2],
        ],
      ],
      [
        [
          [0, 0],
          [-1, 0],
          [-1, -1],
          [0, 2],
          [-1, 2],
        ],
        [[0, 0]],
        [
          [0, 0],
          [-1, 0],
          [-1, -1],
          [0, 2],
          [-1, 2],
        ],
        [],
      ],
    ],
    L: [
      [
        [],
        [
          [0, 0],
          [-1, 0],
          [-1, 1],
          [0, -2],
          [-1, -2],
        ],
        [[0, 0]],
        [
          [0, 0],
          [1, 0],
          [1, 1],
          [0, -2],
          [1, -2],
        ],
      ],
      [
        [
          [0, 0],
          [1, 0],
          [1, -1],
          [0, 2],
          [1, 2],
        ],
        [],
        [
          [0, 0],
          [1, 0],
          [1, -1],
          [0, 2],
          [1, 2],
        ],
        [[0, 0]],
      ],
      [
        [[0, 0]],
        [
          [0, 0],
          [-1, 0],
          [-1, 1],
          [0, -2],
          [-1, -2],
        ],
        [],
        [
          [0, 0],
          [1, 0],
          [1, 1],
          [0, -2],
          [1, -2],
        ],
      ],
      [
        [
          [0, 0],
          [-1, 0],
          [-1, -1],
          [0, 2],
          [-1, 2],
        ],
        [[0, 0]],
        [
          [0, 0],
          [-1, 0],
          [-1, -1],
          [0, 2],
          [-1, 2],
        ],
        [],
      ],
    ],
    J: [
      [
        [],
        [
          [0, 0],
          [-1, 0],
          [-1, 1],
          [0, -2],
          [-1, -2],
        ],
        [[0, 0]],
        [
          [0, 0],
          [1, 0],
          [1, 1],
          [0, -2],
          [1, -2],
        ],
      ],
      [
        [
          [0, 0],
          [1, 0],
          [1, -1],
          [0, 2],
          [1, 2],
        ],
        [],
        [
          [0, 0],
          [1, 0],
          [1, -1],
          [0, 2],
          [1, 2],
        ],
        [[0, 0]],
      ],
      [
        [[0, 0]],
        [
          [0, 0],
          [-1, 0],
          [-1, 1],
          [0, -2],
          [-1, -2],
        ],
        [],
        [
          [0, 0],
          [1, 0],
          [1, 1],
          [0, -2],
          [1, -2],
        ],
      ],
      [
        [
          [0, 0],
          [-1, 0],
          [-1, -1],
          [0, 2],
          [-1, 2],
        ],
        [[0, 0]],
        [
          [0, 0],
          [-1, 0],
          [-1, -1],
          [0, 2],
          [-1, 2],
        ],
        [],
      ],
    ],
    S: [
      [
        [],
        [
          [0, 0],
          [-1, 0],
          [-1, 1],
          [0, -2],
          [-1, -2],
        ],
        [[0, 0]],
        [
          [0, 0],
          [1, 0],
          [1, 1],
          [0, -2],
          [1, -2],
        ],
      ],
      [
        [
          [0, 0],
          [1, 0],
          [1, -1],
          [0, 2],
          [1, 2],
        ],
        [],
        [
          [0, 0],
          [1, 0],
          [1, -1],
          [0, 2],
          [1, 2],
        ],
        [[0, 0]],
      ],
      [
        [[0, 0]],
        [
          [0, 0],
          [-1, 0],
          [-1, 1],
          [0, -2],
          [-1, -2],
        ],
        [],
        [
          [0, 0],
          [1, 0],
          [1, 1],
          [0, -2],
          [1, -2],
        ],
      ],
      [
        [
          [0, 0],
          [-1, 0],
          [-1, -1],
          [0, 2],
          [-1, 2],
        ],
        [[0, 0]],
        [
          [0, 0],
          [-1, 0],
          [-1, -1],
          [0, 2],
          [-1, 2],
        ],
        [],
      ],
    ],
    Z: [
      [
        [],
        [
          [0, 0],
          [-1, 0],
          [-1, 1],
          [0, -2],
          [-1, -2],
        ],
        [[0, 0]],
        [
          [0, 0],
          [1, 0],
          [1, 1],
          [0, -2],
          [1, -2],
        ],
      ],
      [
        [
          [0, 0],
          [1, 0],
          [1, -1],
          [0, 2],
          [1, 2],
        ],
        [],
        [
          [0, 0],
          [1, 0],
          [1, -1],
          [0, 2],
          [1, 2],
        ],
        [[0, 0]],
      ],
      [
        [[0, 0]],
        [
          [0, 0],
          [-1, 0],
          [-1, 1],
          [0, -2],
          [-1, -2],
        ],
        [],
        [
          [0, 0],
          [1, 0],
          [1, 1],
          [0, -2],
          [1, -2],
        ],
      ],
      [
        [
          [0, 0],
          [-1, 0],
          [-1, -1],
          [0, 2],
          [-1, 2],
        ],
        [[0, 0]],
        [
          [0, 0],
          [-1, 0],
          [-1, -1],
          [0, 2],
          [-1, 2],
        ],
        [],
      ],
    ],
  };
}
