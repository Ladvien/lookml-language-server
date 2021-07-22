// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from 'vscode';

import os = require('os');
import path = require('path');
import net = require('net');
import url = require('url');

import { spawn, ChildProcess } from 'child_process';
import { ExtensionContext, workspace, Uri, TextDocument, WorkspaceConfiguration, OutputChannel, window, WorkspaceFolder } from 'vscode';
import { StreamInfo, LanguageClientOptions, LanguageClient, Middleware, ProvideDocumentSymbolsSignature } from 'vscode-languageclient/node';

// this method is called when your extension is activated
// // your extension is activated the very first time the command is executed
// export function activate(context: vscode.ExtensionContext) {

// 	// Use the console to output diagnostic information (console.log) and errors (console.error)
// 	// This line of code will only be executed once when your extension is activated
// 	console.log('Congratulations, your extension "lookml-language-server" is now active!');

// 	// The command has been defined in the package.json file
// 	// Now provide the implementation of the command with registerCommand
// 	// The commandId parameter must match the command field in package.json
// 	let disposable = vscode.commands.registerCommand('lookml-language-server.helloWorld', () => {
// 		// The code you place here will be executed every time your command is executed
// 		// Display a message box to the user
// 		vscode.window.showInformationMessage('Hello World from LookML Language Server!');
// 	});

// 	context.subscriptions.push(disposable);
// }

export function activate(context: vscode.ExtensionContext) {
  let connectionInfo = {
    port: 5007,
    host: "127.0.0.1"
  };

  let serverOptions = () => {
    // Connect to language server via socket
    let socket = net.connect(connectionInfo);
    let result: StreamInfo = {
      writer: socket,
      reader: socket
    };
    return Promise.resolve(result);
  };

  // Options to control the language client
  let clientOptions: LanguageClientOptions = {
    // Register the server for plain text documents
    documentSelector: [{ scheme: 'file', language: 'lkml' }],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher('*.lkml')
    }
  };


  // Create the language client and start the client.
  const client = new LanguageClient(
    'languageServerExample',
    'Language Server Example',
    serverOptions,
    clientOptions
  );

  console.log("Starting client.");
  // Start the client. This will also launch the server
  client.start();
}
// this method is called when your extension is deactivated
export function deactivate() { }
