import { defineCollection } from "astro:content";
import { glob } from "astro/loaders";
import { z } from "zod";

const sourceSchema = z.object({
  title: z.string(),
  url: z.url(),
  publisher: z.string().optional(),
});

const languageSchema = z.object({
  title: z.string(),
  slug: z.string(),
  status: z.enum(["active", "stable", "experimental", "legacy", "inactive"]),
  summary: z.string(),
  firstReleased: z.number().int().positive().optional(),
  creators: z.array(z.string()).default([]),
  paradigms: z.array(z.string()).default([]),
  typing: z.object({
    discipline: z.string(),
    strength: z.string().optional(),
  }),
  memory: z.object({
    model: z.string(),
  }),
  runtime: z.object({
    model: z.string(),
  }),
  officialSite: z.url(),
  repository: z.url().optional(),
  packageManagers: z.array(z.string()).default([]),
  comparedWith: z.array(z.string()).default([]),
  bestFor: z.array(z.string()).default([]),
  poorFit: z.array(z.string()).default([]),
  sources: z.array(sourceSchema).min(1),
  lastVerified: z.coerce.date(),
});

const comparisonSchema = z.object({
  title: z.string(),
  slug: z.string(),
  summary: z.string(),
  languages: z.array(z.string()).min(2),
  useCases: z.array(z.string()).default([]),
  sources: z.array(sourceSchema).min(1),
  lastVerified: z.coerce.date(),
});

const guideSchema = z.object({
  title: z.string(),
  slug: z.string(),
  summary: z.string(),
  audience: z.string().optional(),
  candidates: z.array(z.string()).default([]),
  sources: z.array(sourceSchema).default([]),
  lastVerified: z.coerce.date(),
});

const conceptSchema = z.object({
  title: z.string(),
  slug: z.string(),
  summary: z.string(),
  relatedLanguages: z.array(z.string()).default([]),
  sources: z.array(sourceSchema).default([]),
  lastVerified: z.coerce.date(),
});

export const collections = {
  languages: defineCollection({
    loader: glob({ pattern: "**/*.{md,mdx}", base: "./src/content/languages" }),
    schema: languageSchema,
  }),
  comparisons: defineCollection({
    loader: glob({
      pattern: "**/*.{md,mdx}",
      base: "./src/content/comparisons",
    }),
    schema: comparisonSchema,
  }),
  guides: defineCollection({
    loader: glob({ pattern: "**/*.{md,mdx}", base: "./src/content/guides" }),
    schema: guideSchema,
  }),
  concepts: defineCollection({
    loader: glob({ pattern: "**/*.{md,mdx}", base: "./src/content/concepts" }),
    schema: conceptSchema,
  }),
};
