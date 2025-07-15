import java.util.*;

public class Test {
    // N-Queens Problem
    public static void solveNQueens(int n) {
        int[] queens = new int[n];
        System.out.println("Solving " + n + "-Queens problem:");
        solveNQueensUtil(queens, 0, n);
    }

    private static void solveNQueensUtil(int[] queens, int row, int n) {
        if (row == n) {
            printQueens(queens);
            return;
        }

        for (int col = 0; col < n; col++) {
            if (isSafe(queens, row, col)) {
                queens[row] = col;
                solveNQueensUtil(queens, row + 1, n);
            }
        }
    }

    private static boolean isSafe(int[] queens, int row, int col) {
        for (int i = 0; i < row; i++) {
            if (queens[i] == col ||
                queens[i] - i == col - row ||
                queens[i] + i == col + row) {
                return false;
            }
        }
        return true;
    }

    private static void printQueens(int[] queens) {
        System.out.print("Solution: ");
        for (int i = 0; i < queens.length; i++) {
            System.out.print("(" + i + "," + queens[i] + ") ");
        }
        System.out.println();
    }

    // Sudoku Solver
    public static boolean solveSudoku(int[][] board) {
        for (int row = 0; row < 9; row++) {
            for (int col = 0; col < 9; col++) {
                if (board[row][col] == 0) {
                    for (int num = 1; num <= 9; num++) {
                        if (isValidSudoku(board, row, col, num)) {
                            board[row][col] = num;

                            if (solveSudoku(board)) {
                                return true;
                            }

                            board[row][col] = 0;
                        }
                    }
                    return false;
                }
            }
        }
        return true;
    }

    private static boolean isValidSudoku(int[][] board, int row, int col, int num) {
        // Check row
        for (int j = 0; j < 9; j++) {
            if (board[row][j] == num) return false;
        }

        // Check column
        for (int i = 0; i < 9; i++) {
            if (board[i][col] == num) return false;
        }

        // Check 3x3 box
        int boxRow = row - row % 3;
        int boxCol = col - col % 3;
        for (int i = boxRow; i < boxRow + 3; i++) {
            for (int j = boxCol; j < boxCol + 3; j++) {
                if (board[i][j] == num) return false;
            }
        }

        return true;
    }

    private static void printSudoku(int[][] board) {
        for (int i = 0; i < 9; i++) {
            for (int j = 0; j < 9; j++) {
                System.out.print(board[i][j] + " ");
            }
            System.out.println();
        }
    }

    public static void main(String[] args) {
        // N-Queens
        solveNQueens(4);
        System.out.println();

        // Sudoku
        int[][] sudoku = {
            {5, 3, 0, 0, 7, 0, 0, 0, 0},
            {6, 0, 0, 1, 9, 5, 0, 0, 0},
            {0, 9, 8, 0, 0, 0, 0, 6, 0},
            {8, 0, 0, 0, 6, 0, 0, 0, 3},
            {4, 0, 0, 8, 0, 3, 0, 0, 1},
            {7, 0, 0, 0, 2, 0, 0, 0, 6},
            {0, 6, 0, 0, 0, 0, 2, 8, 0},
            {0, 0, 0, 4, 1, 9, 0, 0, 5},
            {0, 0, 0, 0, 8, 0, 0, 7, 9}
        };

        System.out.println("Sudoku solution:");
        if (solveSudoku(sudoku)) {
            printSudoku(sudoku);
        } else {
            System.out.println("No solution exists");
        }
    }
}
