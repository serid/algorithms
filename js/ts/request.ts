async function main() {
    let result = await fetch("example.com");
    let obj = await result.json();
    console.log(obj);
}

main()