const { chromium } = require('playwright');

(async () => {
  const browser = await chromium.launch();
  const page = await browser.newPage();

  const logs = [];
  page.on('console', msg => logs.push(`[${msg.type()}] ${msg.text()}`));
  page.on('pageerror', err => logs.push(`[ERROR] ${err.message}`));

  console.log('Loading page...');
  await page.goto('http://localhost:9072', { waitUntil: 'networkidle', timeout: 30000 });
  console.log('Page loaded.');
  await page.waitForTimeout(1000);

  console.log('Clicking Run...');
  await page.click('button:has-text("Run")');

  // Poll for output or done status
  for (let i = 0; i < 30; i++) {
    await page.waitForTimeout(1000);
    const output = await page.evaluate(() => {
      const pre = document.querySelector('pre.out');
      return pre ? pre.textContent : '';
    });
    const status = await page.evaluate(() => {
      const el = document.querySelector('.status');
      return el ? el.textContent : '';
    });
    console.log(`[${i+1}s] status: ${status}`);
    if (output.length > 0) {
      console.log(`[${i+1}s] output: ${output.substring(0, 200)}`);
    }
    if (status.includes('done') || status.includes('halted') || status.includes('error') || status.includes('stalled')) {
      console.log('--- FINAL OUTPUT ---');
      console.log(output);
      console.log('--- CONSOLE ---');
      logs.forEach(l => console.log(l));
      await browser.close();
      process.exit(output.includes('HELLO WORLD') ? 0 : 1);
    }
  }

  console.log('TIMEOUT - no completion after 30s');
  const output = await page.evaluate(() => {
    const pre = document.querySelector('pre.out');
    return pre ? pre.textContent : '';
  });
  console.log('output:', output || '(empty)');
  logs.forEach(l => console.log(l));
  await browser.close();
  process.exit(1);
})();
