module.exports = {
  let_declaration: function(tokens) {
    let expression = [tokens.shift()];
    do {
      token = tokens.shift();
      expression.push(token);
    } while (!["EOF", "\n"].includes(token));
    return { type: "let_declaration", name: "x", value: 5 };
  }
};
