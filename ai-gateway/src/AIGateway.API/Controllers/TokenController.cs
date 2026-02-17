using Microsoft.AspNetCore.Mvc;

namespace AIGateway.API.Controllers;

[ApiController]
[Route("api/[controller]")]
public class TokenController : ControllerBase
{
    /// <summary>
    /// Exchange external token for internal token
    /// </summary>
    [HttpPost("exchange")]
    public IActionResult ExchangeToken([FromBody] TokenExchangeRequest request)
    {
        // TODO: Implement token exchange logic with FastGPT
        return Ok(new TokenExchangeResponse
        {
            InternalToken = "generated-internal-token",
            ExpiresIn = 3600
        });
    }

    /// <summary>
    /// Validate internal token
    /// </summary>
    [HttpPost("validate")]
    public IActionResult ValidateToken([FromBody] TokenValidateRequest request)
    {
        // TODO: Implement token validation
        return Ok(new TokenValidateResponse
        {
            IsValid = true,
            UserId = Guid.NewGuid()
        });
    }
}

public record TokenExchangeRequest(string ExternalToken, string? Scope = null);
public record TokenExchangeResponse(string InternalToken, int ExpiresIn);
public record TokenValidateRequest(string Token);
public record TokenValidateResponse(bool IsValid, Guid? UserId = null);
