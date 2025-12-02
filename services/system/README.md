## how to add policy
path: `/usr/share/polkit-1/actions`

```xml
<?xml version="1.0" encoding="UTF-8"?>
<policyconfig>
  <action id="org.mechanix.services.display.getbrightness">
    <description>Get display brightness</description>
    <message>Authentication is required to get display brightness</message>

    <!-- Example: allow active users, require admin if inactive -->
    <defaults>
      <allow_any>auth_admin</allow_any>
      <allow_inactive>auth_admin</allow_inactive>
      <allow_active>auth_admin</allow_active>
    </defaults>
  </action>
</policyconfig>
```