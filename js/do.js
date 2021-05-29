// maybe_monad = {
//     bind: (o, k) =>  {
//         if (o == null)
//             return null
//         else
//             return k(o)
//     }
//     pure: (x) => x
// }

list_monad = {
    bind: (o, k) => o.flatMap(k),
    pure: (x) => [x]
}

function mdo_impl(monad, context, lines, i) {
    if (i >= lines.length) throw new Error("doeg")

    return monad.bind(lines[i][1](context), x => {
        // let new_context = { [line[0]]: x }
        // Object.assign(new_context, context)
        if (i + 1 == lines.length)
            return x
        context[lines[i][0]] = x
        return mdo_impl(monad, context, lines, i + 1)
    })
}

function mdo(monad, ...lines) {
    return mdo_impl(monad, {}, lines, 0)
}

result = mdo(
    list_monad,
    ["x", c => [1, 2]],
    ["y", c => [10, 20]],
    ["_", c => [c.x + c.y]]
)

console.log("Result: " + result)