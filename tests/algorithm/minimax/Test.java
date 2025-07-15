public class Test {
    private static final int PLAYER_X = 1;
    private static final int PLAYER_O = -1;
    private static final int EMPTY = 0;

    public static class Move {
        int row, col, score;

        Move(int row, int col) {
            this.row = row;
            this.col = col;
        }
    }

    public static Move findBestMove(int[][] board) {
        Move bestMove = new Move(-1, -1);
        bestMove.score = Integer.MIN_VALUE;

        for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 3; j++) {
                if (board[i][j] == EMPTY) {
                    board[i][j] = PLAYER_X;
                    int moveScore = minimax(board, 0, false);
                    board[i][j] = EMPTY;

                    if (moveScore > bestMove.score) {
                        bestMove.row = i;
                        bestMove.col = j;
                        bestMove.score = moveScore;
                    }
                }
            }
        }

        return bestMove;
    }

    private static int minimax(int[][] board, int depth, boolean isMaximizing) {
        int score = evaluate(board);

        if (score == 10) return score - depth;
        if (score == -10) return score + depth;
        if (!isMovesLeft(board)) return 0;

        if (isMaximizing) {
            int best = Integer.MIN_VALUE;
            for (int i = 0; i < 3; i++) {
                for (int j = 0; j < 3; j++) {
                    if (board[i][j] == EMPTY) {
                        board[i][j] = PLAYER_X;
                        best = Math.max(best, minimax(board, depth + 1, false));
                        board[i][j] = EMPTY;
                    }
                }
            }
            return best;
        } else {
            int best = Integer.MAX_VALUE;
            for (int i = 0; i < 3; i++) {
                for (int j = 0; j < 3; j++) {
                    if (board[i][j] == EMPTY) {
                        board[i][j] = PLAYER_O;
                        best = Math.min(best, minimax(board, depth + 1, true));
                        board[i][j] = EMPTY;
                    }
                }
            }
            return best;
        }
    }

    private static int evaluate(int[][] board) {
        // Check rows
        for (int row = 0; row < 3; row++) {
            if (board[row][0] == board[row][1] && board[row][1] == board[row][2]) {
                if (board[row][0] == PLAYER_X) return 10;
                else if (board[row][0] == PLAYER_O) return -10;
            }
        }

        // Check columns
        for (int col = 0; col < 3; col++) {
            if (board[0][col] == board[1][col] && board[1][col] == board[2][col]) {
                if (board[0][col] == PLAYER_X) return 10;
                else if (board[0][col] == PLAYER_O) return -10;
            }
        }

        // Check diagonals
        if (board[0][0] == board[1][1] && board[1][1] == board[2][2]) {
            if (board[0][0] == PLAYER_X) return 10;
            else if (board[0][0] == PLAYER_O) return -10;
        }

        if (board[0][2] == board[1][1] && board[1][1] == board[2][0]) {
            if (board[0][2] == PLAYER_X) return 10;
            else if (board[0][2] == PLAYER_O) return -10;
        }

        return 0;
    }

    private static boolean isMovesLeft(int[][] board) {
        for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 3; j++) {
                if (board[i][j] == EMPTY) return true;
            }
        }
        return false;
    }

    private static void printBoard(int[][] board) {
        for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 3; j++) {
                char symbol = board[i][j] == PLAYER_X ? 'X' :
                             board[i][j] == PLAYER_O ? 'O' : '-';
                System.out.print(symbol + " ");
            }
            System.out.println();
        }
        System.out.println();
    }

    public static void main(String[] args) {
        int[][] board = {
            {PLAYER_X, PLAYER_O, PLAYER_X},
            {PLAYER_O, PLAYER_O, PLAYER_X},
            {EMPTY, EMPTY, EMPTY}
        };

        System.out.println("Current board:");
        printBoard(board);

        Move bestMove = findBestMove(board);
        System.out.println("Best move for X: (" + bestMove.row + ", " + bestMove.col + ")");
        System.out.println("Move score: " + bestMove.score);
    }
}

