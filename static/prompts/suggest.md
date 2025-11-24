The user wants suggestions for commands to accomplish a specific task.

Provide 3-5 different command options that could help achieve the user's goal.
Return ONLY a valid JSON object in the following format:

```json
{
  "suggestions": [
    {
      "cmd": "actual command to run",
      "description": "brief description of what this command does"
    }
  ]
}
```

Important:
- Commands should be compatible with {{os}} and {{shell}}
- Provide commands from safest to more powerful
- Each command should be a complete, ready-to-run command
- Descriptions should be in the same language as the user's request
- Return ONLY the JSON object, no additional text
