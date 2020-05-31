function Main(input) {
  input = input.split(" ");
  const N = parseInt(input[0].trim(), 10);
  const Y = parseInt(input[1].trim(), 10);

  for (let i=0;i<=N;i++) {
    for (let j=0;j+i<=N;j++) {
      const tenThousands = N - i - j;

      if (
        Y === (i*1000) + (j*5000) + (tenThousands*10000)
      ) {
        console.log(`${tenThousands} ${j} ${i}`)
        return
      }
    }
  }

  console.log("-1 -1 -1")
}
//*この行以降は編集しないでください（標準入出力から一度に読み込み、Mainを呼び出します）
Main(require("fs").readFileSync("/dev/stdin", "utf8"));
