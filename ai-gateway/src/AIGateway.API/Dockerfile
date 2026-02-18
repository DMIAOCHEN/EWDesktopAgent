# Stage 1: Build
FROM mcr.microsoft.com/dotnet/sdk:8.0 AS build
WORKDIR /src

# Copy solution and project files
COPY ["AIGateway.sln", "./"]
COPY ["src/AIGateway.Domain/AIGateway.Domain.csproj", "src/AIGateway.Domain/"]
COPY ["src/AIGateway.Infrastructure/AIGateway.Infrastructure.csproj", "src/AIGateway.Infrastructure/"]
COPY ["src/AIGateway.API/AIGateway.API.csproj", "src/AIGateway.API/"]

# Restore dependencies
RUN dotnet restore "src/AIGateway.API/AIGateway.API.csproj"

# Copy all source code
COPY . .

# Build the application
WORKDIR "/src/src/AIGateway.API"
RUN dotnet build "AIGateway.API.csproj" -c Release -o /app/build

# Stage 2: Publish
FROM build AS publish
RUN dotnet publish "AIGateway.API.csproj" -c Release -o /app/publish /p:UseAppHost=false

# Stage 3: Runtime
FROM mcr.microsoft.com/dotnet/aspnet:8.0 AS final
WORKDIR /app

# Install timezone data and set timezone
RUN apt-get update && apt-get install -y --no-install-recommends tzdata \
    && ln -sf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime \
    && echo "Asia/Shanghai" > /etc/timezone \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Copy published files
COPY --from=publish /app/publish .

# Expose ports
EXPOSE 80
EXPOSE 443

# Set environment variables
ENV ASPNETCORE_ENVIRONMENT=Production
ENV TZ=Asia/Shanghai

# Entry point
ENTRYPOINT ["dotnet", "AIGateway.API.dll"]
