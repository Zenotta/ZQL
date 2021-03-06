![ZQL Logo](https://github.com/zenotta/zql/blob/master/assets/logo.png "ZQL Logo")

# The Zenotta Query Language (ZQL)

ZQL is a dynamically typed, declarative language that runs expressions on the blockchain, currently only on the Zenotta platform. The vision for ZQL is that the language will run on any blockchain system and with any Turing complete language, but we'll need your help to get there!

The Zenotta Query Language is composed of two components: the Heap and the Stack.

## Table of Contents

- [The ZQL Heap](https://github.com/zenotta/zql#the-zql-heap)
- [The ZQL Stack](https://github.com/zenotta/zql#the-zql-stack)
- [Development](https://github.com/zenotta/zql#development)
- [License](https://github.com/zenotta/zql#license)

## The ZQL Heap

The ZQL Heap is the Turing complete component of the language, capable of running various logic operations in order to determine what 
transactions will eventually be executed on the Zenotta blockchain. ZQL comes with its own Heap language, but in the future the logic 
of the language can be written in any Turing complete language, including the ones you know and love like C, Python or Solidity.

## The ZQL Stack

The ZQL Stack is the set of transaction queries that will be executed on the blockchain. As a set of queries, the Stack is not Turing 
complete and looks more like the SQL you may be familiar with. Although the Stack can only execute transactions on the Zenotta blockchain at present, the vision is to have transactions execute on any blockchain system, including Bitcoin, Litecoin and Ripple.

## Development

ZQL is currently under development and always open to contributions! 

## License

ZQL is licensed under the [MIT License](https://github.com/zenotta/zql/blob/master/LICENSE.txt).

