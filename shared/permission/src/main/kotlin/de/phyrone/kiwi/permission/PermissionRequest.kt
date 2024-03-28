package de.phyrone.kiwi.permission

sealed interface PermissionRequest {
    data class GlobalPermissionRequest(val permission: String) : PermissionRequest
    data class ProfileScopedPermissionRequest(val permission: String, val profile: Long) : PermissionRequest
    data class GuildScopedPermissionRequest(val permission: String, val guild: Long) : PermissionRequest
    data class ChannelScopedPermissionRequest(val permission: String, val channel: Long) : PermissionRequest

}