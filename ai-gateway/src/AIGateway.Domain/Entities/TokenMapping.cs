namespace AIGateway.Domain.Entities;

/// <summary>
/// Represents a token mapping between external tokens and internal tokens
/// </summary>
public class TokenMapping
{
    public Guid Id { get; set; }
    public string ExternalToken { get; set; } = string.Empty;
    public string InternalToken { get; set; } = string.Empty;
    public Guid UserId { get; set; }
    public DateTime CreatedAt { get; set; }
    public DateTime ExpiresAt { get; set; }
    public bool IsActive { get; set; }
}

/// <summary>
/// Represents a user in the system
/// </summary>
public class User
{
    public Guid Id { get; set; }
    public string Username { get; set; } = string.Empty;
    public string Email { get; set; } = string.Empty;
    public string PasswordHash { get; set; } = string.Empty;
    public Guid? InstitutionId { get; set; }
    public DateTime CreatedAt { get; set; }
    public DateTime? LastLoginAt { get; set; }
    public bool IsActive { get; set; }
}

/// <summary>
/// Represents an institution
/// </summary>
public class Institution
{
    public Guid Id { get; set; }
    public string Name { get; set; } = string.Empty;
    public string Code { get; set; } = string.Empty;
    public DateTime CreatedAt { get; set; }
    public bool IsActive { get; set; }
}

/// <summary>
/// Represents a business system (RIS/PIS/EIS)
/// </summary>
public class BusinessSystem
{
    public Guid Id { get; set; }
    public string Name { get; set; } = string.Empty;
    public string Code { get; set; } = string.Empty;
    public string BaseUrl { get; set; } = string.Empty;
    public Guid InstitutionId { get; set; }
    public DateTime CreatedAt { get; set; }
    public bool IsActive { get; set; }
}
