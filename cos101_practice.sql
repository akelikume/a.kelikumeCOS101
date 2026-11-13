--
-- PostgreSQL database dump
--

\restrict JqLg6fXhFYoXErJwCoPGFJP8PrH6yG6l7mA4Fl5R8g3ja9LBJtEJHmSkv2KJxnD

-- Dumped from database version 18.7
-- Dumped by pg_dump version 18.7

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: company; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.company (
    id integer NOT NULL,
    name text NOT NULL,
    age integer NOT NULL,
    address character(50),
    salary real,
    join_date date
);


ALTER TABLE public.company OWNER TO postgres;

--
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    id integer NOT NULL,
    dept character(50) NOT NULL,
    emp_id integer NOT NULL
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Data for Name: company; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.company (id, name, age, address, salary, join_date) FROM stdin;
1	Chuka	32	Ebonyi                                            	20000	2001-07-13
2	Ethel	22	Edo                                               	\N	2007-12-15
3	Adenike	27	Osun                                              	30000	\N
4	Hassan	31	Kano                                              	90000	2007-12-01
5	David	23	Calabar                                           	85000	2007-11-11
6	Chisom	33	Enugu                                             	45000	\N
\.


--
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (id, dept, emp_id) FROM stdin;
\.


--
-- Name: company company_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.company
    ADD CONSTRAINT company_pkey PRIMARY KEY (id);


--
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (id);


--
-- PostgreSQL database dump complete
--

\unrestrict JqLg6fXhFYoXErJwCoPGFJP8PrH6yG6l7mA4Fl5R8g3ja9LBJtEJHmSkv2KJxnD

