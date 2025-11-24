Analyze the user's input and determine which mode is most appropriate.

Return ONLY a valid JSON object in the following format:

```json
{
  "mode": "ask|suggest|execute",
  "reason": "brief explanation in the same language as the input"
}
```

Guidelines:
- "ask": For general questions, explanations, or when the user wants to learn something
- "suggest": When the user wants options or isn't sure about the exact command
- "execute": When the user clearly wants to perform a specific action

Return ONLY the JSON object, no additional text.
