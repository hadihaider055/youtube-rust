// let abc = { a: undefined, b: "hello" };
// let cde: string = abc.a;

let abc = null;
abc?.map((item) => console.log(item));

// console.log(abc);

try {
  // const fileContent = fs.readFileSync("example.txt", "utf-8");
  // console.log("file content: ", fileContent);
} catch (err) {
  // fs.writeFile("example.txt", "creating file programatically!", () => {
  //   console.log("File created successfully!");
  //   return;
  // });
  //   console.log("error", err);
}
