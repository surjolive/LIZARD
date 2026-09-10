import * as vscode from 'vscode';
import { execFile } from 'node:child_process';

function executablePath(): string {
  return vscode.workspace.getConfiguration('lizard').get<string>('path', 'lizard');
}

function runCli(args: string[], cwd?: string): Promise<{ stdout: string; stderr: string }> {
  return new Promise((resolve, reject) => {
    execFile(executablePath(), args, { cwd }, (error, stdout, stderr) => {
      if (error && !stdout && !stderr) {
        reject(error);
        return;
      }
      resolve({ stdout, stderr });
    });
  });
}

export function activate(context: vscode.ExtensionContext): void {
  const formatter = vscode.languages.registerDocumentFormattingEditProvider('lizard', {
    async provideDocumentFormattingEdits(document) {
      const result = await runCli(['fmt', document.uri.fsPath], vscode.workspace.getWorkspaceFolder(document.uri)?.uri.fsPath);
      if (result.stderr && !result.stdout) {
        throw new Error(result.stderr);
      }
      const fullRange = document.validateRange(new vscode.Range(0, 0, document.lineCount, 0));
      return [vscode.TextEdit.replace(fullRange, result.stdout)];
    }
  });

  const runFile = vscode.commands.registerCommand('lizard.runFile', async () => {
    const editor = vscode.window.activeTextEditor;
    if (!editor) return;
    const terminal = vscode.window.createTerminal('LIZARD');
    terminal.show();
    terminal.sendText(`${executablePath()} "${editor.document.uri.fsPath}"`);
  });

  const checkFile = vscode.commands.registerCommand('lizard.checkFile', async () => {
    const editor = vscode.window.activeTextEditor;
    if (!editor) return;
    const result = await runCli(['check', editor.document.uri.fsPath]);
    if (result.stderr) vscode.window.showErrorMessage(result.stderr.trim());
    else vscode.window.showInformationMessage(result.stdout.trim());
  });

  const formatFile = vscode.commands.registerCommand('lizard.formatFile', () => vscode.commands.executeCommand('editor.action.formatDocument'));
  context.subscriptions.push(formatter, runFile, checkFile, formatFile);
}

export function deactivate(): void {}
