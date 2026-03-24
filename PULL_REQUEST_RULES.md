# Branch Protection & Pull Request Rules

## main branch

- ✅ Require pull request before merging
- ✅ Required approvals: **1**
- ✅ Required reviewer: **zacxxx**
- ✅ Dismiss stale reviews when new commits are pushed
- ✅ Require review from code owners
- ❌ Allow bypassing the above settings: **disabled**
- ❌ Allow force pushes: **disabled**
- ❌ Allow deletions: **disabled**
- ❌ Allow direct commits: **disabled**

## dev branch

- ✅ Require pull request before merging
- ✅ Required approvals: 1
- ❌ Allow force pushes: disabled

## Code Owners

```
# .github/CODEOWNERS
* @zacxxx
```

All files are owned by `zacxxx`. Any PR touching any file requires their approval.
