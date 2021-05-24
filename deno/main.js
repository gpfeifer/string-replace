// Run with deno --allow-read --allow-hrtime main.js 
const text = Deno.readTextFileSync('../string.json');
const t0 = performance.now();
const result = text.replaceAll("http://localhost:35261/product", "http://example.com/product");
const t1 = performance.now();
console.log(`Reaplace:  ${t1 - t0} ms`);
