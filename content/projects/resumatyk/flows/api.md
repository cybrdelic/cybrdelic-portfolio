# API Documentation

Resumatyk offers a comprehensive REST API that allows developers to integrate resume generation capabilities into their applications. This documentation covers all available endpoints, authentication methods, and usage examples.

## Authentication

### API Keys

To use the Resumatyk API, you need an API key:

1. Register at [developer.resumatyk.com](https://developer.resumatyk.com)
2. Create a new API key in your developer dashboard
3. Use the key in all API requests

Example authentication:

```bash
curl -X GET https://api.resumatyk.com/v1/templates \
  -H "Authorization: Bearer YOUR_API_KEY"
```

### Rate Limiting

Standard API plans have the following limits:

| Plan | Requests/minute | Resumes/day | Templates |
|------|----------------|-------------|-----------|
| Free | 10 | 5 | Standard only |
| Basic | 50 | 100 | All public |
| Premium | 200 | Unlimited | All + custom |
| Enterprise | Custom | Custom | Custom |

Rate limit headers are included in all responses:

```
X-RateLimit-Limit: 50
X-RateLimit-Remaining: 49
X-RateLimit-Reset: 1625097600
```

## Core Endpoints

### Resume Operations

#### Create Resume

```
POST /v1/resumes
```

Create a new resume from provided content:

```json
{
  "name": "John Doe Resume",
  "template_id": "modern-professional",
  "content": {
    "personal_info": {
      "name": "John Doe",
      "email": "john@example.com",
      "phone": "+1 555-123-4567",
      "location": "San Francisco, CA",
      "linkedin": "linkedin.com/in/johndoe"
    },
    "summary": "Experienced software engineer with 8 years...",
    "experience": [
      {
        "company": "Tech Corp",
        "position": "Senior Developer",
        "location": "San Francisco, CA",
        "dates": "2018-Present",
        "achievements": [
          "Led development of microservice architecture...",
          "Improved system performance by 40%..."
        ]
      }
    ],
    "education": [
      {
        "institution": "University of California",
        "degree": "B.S. Computer Science",
        "dates": "2010-2014"
      }
    ],
    "skills": [
      "JavaScript", "React", "Node.js", "Python"
    ]
  }
}
```

Response:

```json
{
  "resume_id": "f7c2a4d1-b8e5-4b9c-8e76-2e4f8cb4b23a",
  "created_at": "2023-07-23T14:32:10Z",
  "status": "processing",
  "links": {
    "self": "/v1/resumes/f7c2a4d1-b8e5-4b9c-8e76-2e4f8cb4b23a",
    "status": "/v1/resumes/f7c2a4d1-b8e5-4b9c-8e76-2e4f8cb4b23a/status"
  }
}
```

#### Get Resume Status

```
GET /v1/resumes/{resume_id}/status
```

Check the status of a resume generation request:

```json
{
  "resume_id": "f7c2a4d1-b8e5-4b9c-8e76-2e4f8cb4b23a",
  "status": "completed",
  "progress": 100,
  "links": {
    "download": "/v1/resumes/f7c2a4d1-b8e5-4b9c-8e76-2e4f8cb4b23a/download"
  }
}
```

Possible status values:
- `processing`: Resume is being generated
- `completed`: Resume is ready for download
- `failed`: Generation failed (see `error` field)
- `queued`: Request is waiting for processing

#### Download Resume

```
GET /v1/resumes/{resume_id}/download?format={format}
```

Download a generated resume in the specified format:

Supported formats:
- `pdf` (default)
- `docx`
- `tex` (LaTeX source)
- `json` (structured data)

Response:
- Binary file download
- Content-Type depends on format requested

#### List Resumes

```
GET /v1/resumes?page={page}&limit={limit}
```

List all resumes created by your account:

```json
{
  "page": 1,
  "limit": 10,
  "total": 42,
  "resumes": [
    {
      "resume_id": "f7c2a4d1-b8e5-4b9c-8e76-2e4f8cb4b23a",
      "name": "John Doe Resume",
      "created_at": "2023-07-23T14:32:10Z",
      "template_id": "modern-professional",
      "status": "completed"
    },
    {
      "resume_id": "a1b2c3d4-e5f6-7g8h-9i0j-k1l2m3n4o5p6",
      "name": "John Doe Technical Resume",
      "created_at": "2023-07-22T09:15:30Z",
      "template_id": "tech-minimal",
      "status": "completed"
    }
  ]
}
```

### Template Operations

#### List Templates

```
GET /v1/templates?category={category}
```

List available resume templates:

```json
{
  "templates": [
    {
      "id": "modern-professional",
      "name": "Modern Professional",
      "category": "professional",
      "thumbnail_url": "https://api.resumatyk.com/thumbnails/modern-professional.png",
      "description": "Clean, professional template with modern typography"
    },
    {
      "id": "tech-minimal",
      "name": "Tech Minimal",
      "category": "technical",
      "thumbnail_url": "https://api.resumatyk.com/thumbnails/tech-minimal.png",
      "description": "Minimalist template optimized for technical roles"
    }
  ]
}
```

Optional query parameters:
- `category`: Filter by template category (professional, creative, academic, technical)

#### Get Template Details

```
GET /v1/templates/{template_id}
```

Get detailed information about a specific template:

```json
{
  "id": "modern-professional",
  "name": "Modern Professional",
  "category": "professional",
  "thumbnail_url": "https://api.resumatyk.com/thumbnails/modern-professional.png",
  "description": "Clean, professional template with modern typography",
  "features": [
    "Two-column layout",
    "Custom header with contact information",
    "Skill visualization",
    "ATS-friendly"
  ],
  "recommended_for": [
    "Business professionals",
    "Mid-career candidates",
    "Corporate applications"
  ],
  "preview_url": "https://api.resumatyk.com/previews/modern-professional.pdf"
}
```

### AI Enhancement

#### Enhance Description

```
POST /v1/enhance/description
```

Transform a basic job description into an achievement-focused bullet point:

```json
{
  "description": "Managed a team of developers",
  "tone": "professional",
  "include_metrics": true
}
```

Response:

```json
{
  "enhanced": [
    "Led a high-performing team of 5 developers, delivering 12 projects on time and under budget with a 98% client satisfaction rate",
    "Directed cross-functional development team, increasing productivity by 34% through implementation of agile methodologies and improved workflows",
    "Managed team of developers to successfully deliver mission-critical applications, reducing time-to-market by 40% compared to previous releases"
  ]
}
```

#### Analyze Job Description

```
POST /v1/analyze/job
```

Extract key requirements and keywords from a job description:

```json
{
  "job_description": "We are looking for a Senior Software Engineer with 5+ years of experience in Python and cloud technologies. The ideal candidate has experience with AWS, microservices architecture, and CI/CD pipelines. Experience with machine learning is a plus."
}
```

Response:

```json
{
  "required_skills": [
    {"skill": "Python", "importance": 0.9},
    {"skill": "AWS", "importance": 0.8},
    {"skill": "Microservices", "importance": 0.7},
    {"skill": "CI/CD", "importance": 0.7}
  ],
  "preferred_skills": [
    {"skill": "Machine Learning", "importance": 0.5}
  ],
  "experience_level": "Senior",
  "years_required": 5,
  "keywords": [
    "cloud technologies",
    "software engineer",
    "microservices architecture",
    "pipelines"
  ]
}
```

#### Resume Optimization

```
POST /v1/optimize
```

Optimize a resume for a specific job description:

```json
{
  "resume_id": "f7c2a4d1-b8e5-4b9c-8e76-2e4f8cb4b23a",
  "job_description": "We are looking for a Senior Software Engineer...",
  "optimization_level": "high"
}
```

Response:

```json
{
  "optimization_id": "o1p2q3r4-s5t6-u7v8-w9x0-y1z2a3b4c5d6",
  "status": "processing",
  "links": {
    "status": "/v1/optimize/o1p2q3r4-s5t6-u7v8-w9x0-y1z2a3b4c5d6"
  }
}
```

## Error Handling

All API errors follow this format:

```json
{
  "error": {
    "code": "invalid_template",
    "message": "The requested template ID does not exist",
    "details": {
      "template_id": "non-existent-template"
    }
  }
}
```

Common error codes:

| Code | Description | HTTP Status |
|------|-------------|-------------|
| `authentication_error` | Invalid or missing API key | 401 |
| `rate_limit_exceeded` | Too many requests | 429 |
| `invalid_request` | Malformed request | 400 |
| `resource_not_found` | Resource does not exist | 404 |
| `processing_error` | Error during resume generation | 500 |
| `insufficient_permissions` | Plan does not allow this operation | 403 |

## Webhooks

For asynchronous operations, set up webhooks:

### Register Webhook

```
POST /v1/webhooks
```

```json
{
  "url": "https://your-app.com/webhook",
  "events": ["resume.completed", "resume.failed"]
}
```

Response:

```json
{
  "webhook_id": "wh_123456",
  "url": "https://your-app.com/webhook",
  "events": ["resume.completed", "resume.failed"],
  "created_at": "2023-07-23T14:32:10Z"
}
```

### Webhook Events

When an event occurs, we'll send a POST request to your webhook URL:

```json
{
  "event": "resume.completed",
  "created_at": "2023-07-23T14:40:23Z",
  "data": {
    "resume_id": "f7c2a4d1-b8e5-4b9c-8e76-2e4f8cb4b23a",
    "status": "completed",
    "links": {
      "download": "/v1/resumes/f7c2a4d1-b8e5-4b9c-8e76-2e4f8cb4b23a/download"
    }
  }
}
```

## SDK Support

Official client libraries:

- **JavaScript/TypeScript**: [npm package](https://www.npmjs.com/package/resumatyk-client)
- **Python**: [PyPI package](https://pypi.org/project/resumatyk/)
- **Ruby**: [RubyGems package](https://rubygems.org/gems/resumatyk)
- **PHP**: [Composer package](https://packagist.org/packages/resumatyk/client)

## Example Integration

Simple Node.js example for generating a resume:

```javascript
const Resumatyk = require('resumatyk-client');

const client = new Resumatyk('YOUR_API_KEY');

async function generateResume() {
  try {
    // Create resume
    const resumeData = {
      name: "John Doe Resume",
      template_id: "modern-professional",
      content: {
        personal_info: {
          name: "John Doe",
          email: "john@example.com",
          // ...other fields
        },
        // ...other sections
      }
    };
    
    const { resume_id } = await client.resumes.create(resumeData);
    
    // Poll for completion
    let status;
    do {
      await new Promise(resolve => setTimeout(resolve, 1000));
      status = await client.resumes.getStatus(resume_id);
    } while (status.status === 'processing' || status.status === 'queued');
    
    if (status.status === 'completed') {
      // Download the PDF
      const pdfBuffer = await client.resumes.download(resume_id, 'pdf');
      console.log(`Resume generated! Size: ${pdfBuffer.length} bytes`);
      return pdfBuffer;
    } else {
      console.error('Resume generation failed:', status.error);
    }
  } catch (error) {
    console.error('API Error:', error);
  }
}

generateResume();
```