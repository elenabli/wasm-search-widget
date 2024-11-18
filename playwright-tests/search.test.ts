import { test, expect } from '@playwright/test';

test('show the only kanji with 23 strokes', async ({ page }) => {
  // Navigate to the target page
  await page.goto('/');

  // Input the search term
  await page.fill('#filter', '23');

  // Check that table has right amount of rows
  const tr = page.locator('table tbody tr');
  await tr.waitFor();
  expect((await tr.all()).length).toBe(1);
});

test('person has both readings available', async ({ page }) => {
  // Navigate to the target page
  await page.goto('/');

  // Input the search term
  await page.fill('#filter', '2');

  // Wait for the table to load with at least 10 rows
  const tr = page.locator('table tbody tr:nth-child(10)');
  await tr.waitFor();
  
  // Check that table has right amount of rows
  expect((await page.locator('table tbody tr').all()).length).toBe(12);

  // Find the row with the kanji for person
  const cell = page.getByRole('cell', { name: '人' });
  const row = page.getByRole('row').filter({ has: cell });

  // Onyomi should be present
  await expect(row).toContainText('jin');
  await expect(row).toContainText('nin');

  // Kunyomi should be present
  await expect(row).toContainText('hito');
});