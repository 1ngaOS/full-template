import { test, expect } from './helpers/setup';

test('homepage loads and displays content', async ({ page }) => {
  await page.goto('/');
  await expect(page.locator('h1')).toContainText('Sample SvelteKit Application');
});

test('button click increments counter', async ({ page }) => {
  await page.goto('/');
  const button = page.locator('button');
  await expect(button).toContainText('Click me! (0)');
  await button.click();
  await expect(button).toContainText('Click me! (1)');
});

