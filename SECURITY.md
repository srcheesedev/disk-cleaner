# Security Policy

## Supported Versions

Currently supported versions of Disk Cleaner:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability in Disk Cleaner, please report it responsibly:

1. **DO NOT** create a public GitHub issue
2. **DO NOT** post details on social media or forums

Instead, please:

### Contact
- Email: [REPLACE_WITH_YOUR_EMAIL]
- Subject: "Security Vulnerability - Disk Cleaner"

### Information to Include
- Description of the vulnerability
- Steps to reproduce the issue
- Potential impact assessment
- Suggested fix (if you have one)

### Response Timeline
- **48 hours**: Acknowledgment of your report
- **1 week**: Initial assessment and communication
- **2 weeks**: Estimated fix timeline
- **4 weeks**: Security patch release (if needed)

## Security Considerations

### File Deletion Safety
- Always reviews selections before confirming deletion
- Files are permanently deleted - ensure backups exist
- No recovery mechanism is provided by the tool

### Permissions
- Tool respects file system permissions
- Cannot delete files without proper access rights
- No privilege escalation is performed

### Data Privacy
- No data is transmitted over networks
- No user data is collected or stored
- All operations are performed locally

## Best Practices for Users

1. **Always backup important data** before using the tool
2. **Test in non-critical directories** first
3. **Review file selections carefully** before confirming deletion
4. **Use version control** for project directories
5. **Understand file permissions** before running as different users

## Known Security Limitations

1. **No file recovery**: Deleted files are permanently removed
2. **Race conditions**: Files can be modified between analysis and deletion
3. **Symlink handling**: Tool follows symlinks (potential security consideration)
4. **Large directories**: Memory usage scales with directory size

## Secure Development

This project follows secure development practices:

- **Memory safety**: Built in Rust to prevent buffer overflows
- **Input validation**: All user inputs are validated
- **Error handling**: Graceful failure without information leakage
- **Test coverage**: Comprehensive test suite including edge cases
- **Dependencies**: Regular updates to avoid known vulnerabilities

## Vulnerability Disclosure Timeline

When a security issue is confirmed:

1. **Day 0**: Vulnerability confirmed
2. **Day 1-7**: Development of fix
3. **Day 7-14**: Testing and validation
4. **Day 14**: Public disclosure and patch release
5. **Day 14+**: Communication to users about the update

## Contact

For non-security related issues, please use GitHub issues.
For security concerns, use the email contact method above.

Thank you for helping keep Disk Cleaner secure!