-- Fix by_grade column: MSSQL has correct per-item grades, PostgreSQL export lost them.
-- Knight Online convention: last digit of item num = upgrade grade.
UPDATE item SET by_grade = (num % 10)::SMALLINT WHERE by_grade = 0 AND num % 10 != 0;
